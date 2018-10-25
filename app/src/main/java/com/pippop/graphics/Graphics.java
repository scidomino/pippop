package com.pippop.graphics;

import android.content.Context;
import android.graphics.Typeface;
import android.opengl.GLES20;
import android.support.v4.content.res.ResourcesCompat;
import android.view.MotionEvent;
import com.pippop.R;
import com.pippop.graph.Point;
import com.pippop.graphics.gltext.GLText;
import com.pippop.graphics.program.BatchTextureProgram;
import com.pippop.graphics.program.GlowProgram;
import com.pippop.graphics.program.StandardProgram;
import java.io.InputStream;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.nio.FloatBuffer;
import java.util.Scanner;

public class Graphics {
  private static final int FLOAT_BYTES = 4;
  private static final int VIRTUAL_WIDTH = 300;

  private final StandardProgram standardProgram;
  private final GlowProgram glowProgram;

  private final float[] transformMatrix = new float[4];
  private final GLText glText;
  private final GLText glTextOutline;
  private float width;
  private float height;

  public Graphics(Context context) {
    this.standardProgram = new StandardProgram(context);
    this.glowProgram = new GlowProgram(context);
    BatchTextureProgram batchTextureProgram = new BatchTextureProgram(context);
    Typeface font = ResourcesCompat.getFont(context, R.font.sniglet_extrabold);
    glText = new GLText(batchTextureProgram, font, 30, 2, 2, false);
    glTextOutline = new GLText(batchTextureProgram, font, 30, 2, 2, true);
  }

  private static int loadProgram(Context context, int fragmentShader, int vertexShader) {
    int program = GLES20.glCreateProgram();
    GLES20.glAttachShader(program, loadShader(GLES20.GL_VERTEX_SHADER, context, vertexShader));
    GLES20.glAttachShader(program, loadShader(GLES20.GL_FRAGMENT_SHADER, context, fragmentShader));
    GLES20.glLinkProgram(program);

    final int[] status = new int[1];
    GLES20.glGetProgramiv(program, GLES20.GL_LINK_STATUS, status, 0);
    if (status[0] == 0) {
      throw new RuntimeException("Error creating program: " + GLES20.glGetProgramInfoLog(program));
    }
    return program;
  }

  private static int loadShader(int type, Context context, int resourceId) {
    int shader = GLES20.glCreateShader(type);
    GLES20.glShaderSource(shader, loadResource(context, resourceId));
    GLES20.glCompileShader(shader);

    int[] status = new int[1];
    GLES20.glGetShaderiv(shader, GLES20.GL_COMPILE_STATUS, status, 0);
    if (status[0] == 0) {
      String log = GLES20.glGetShaderInfoLog(shader);
      throw new RuntimeException("Could not compile shader : " + log);
    }
    return shader;
  }

  private static String loadResource(Context context, int resourceId) {
    InputStream stream = context.getResources().openRawResource(resourceId);
    return new Scanner(stream, "UTF-8").useDelimiter("\\A").next();
  }

  public static FloatBuffer createVertexBuffer(int size) {
    return ByteBuffer.allocateDirect(size * FLOAT_BYTES)
        .order(ByteOrder.nativeOrder())
        .asFloatBuffer();
  }

  public void updateDimensions(int width, int height) {
    this.width = width;
    this.height = height;
    GLES20.glViewport(0, 0, width, height);
    float ratio = (float) width / height;
    transformMatrix[0] = 1f / VIRTUAL_WIDTH;
    transformMatrix[3] = ratio / VIRTUAL_WIDTH;
  }

  public Point convertToBubbleSpacePoint(MotionEvent point) {
    float glX = (2 * point.getX() / width) - 1;
    float glY = -((2 * point.getY() / height) - 1);
    return new Point(glX / transformMatrix[0], glY / transformMatrix[3]);
  }

  public void drawLine(GlowLine line, Color color) {
    GLES20.glEnable(GLES20.GL_BLEND);
    GLES20.glBlendFunc(GLES20.GL_SRC_ALPHA, GLES20.GL_ONE_MINUS_SRC_ALPHA);
    glowProgram.draw(line.getBuffer(), color, transformMatrix);
    GLES20.glDisable(GLES20.GL_BLEND);
  }

  public void drawString(String value, Color fillColor, Color outineColor, float x, float y) {
    float[] mVPMatrix = new float[16];
    mVPMatrix[0] = transformMatrix[0];
    mVPMatrix[5] = transformMatrix[3];
    mVPMatrix[10] = 1;
    mVPMatrix[15] = 1;
    GLES20.glEnable(GLES20.GL_BLEND);
    GLES20.glBlendFunc(GLES20.GL_ONE, GLES20.GL_ONE_MINUS_SRC_ALPHA);

    glText.begin(fillColor, mVPMatrix);
    glText.drawC(value, x, y, 0f);
    glText.end();

    GLES20.glBlendFunc(GLES20.GL_ONE, GLES20.GL_ONE_MINUS_SRC_ALPHA);
    glTextOutline.begin(outineColor, mVPMatrix);
    glTextOutline.drawC(value, x, y, 0f);
    glTextOutline.end();

    GLES20.glDisable(GLES20.GL_BLEND);
  }

  public void drawFill(FloatBuffer buffer, Color color) {
    standardProgram.draw(buffer, color, GLES20.GL_TRIANGLE_FAN, 0, 0, transformMatrix);
  }

  public void draw(FloatBuffer buffer, Color color, float width) {
    GLES20.glLineWidth(width);
    standardProgram.draw(buffer, color, GLES20.GL_LINE_LOOP, 1, 1, transformMatrix);
  }
}
