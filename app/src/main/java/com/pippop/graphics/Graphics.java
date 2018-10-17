package com.pippop.graphics;

import android.content.Context;
import android.opengl.GLES20;
import com.pippop.R;
import com.pippop.graph.Point;
import java.io.InputStream;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.nio.FloatBuffer;
import java.util.Scanner;

public class Graphics {
  private static final int FLOAT_BYTES = 4;
  private static final int COORDS_PER_VERTEX = 2;
  private static final int VIRTUAL_WIDTH = 300;

  private final int standardProgram;
  private final int colorHandle;
  private final int posHandle;
  private final int matrixHandle;

  private final float[] transformMatrix = new float[4];
  private float width;
  private float height;

  public Graphics(Context context) {
    this.standardProgram = loadProgram(context);
    this.colorHandle = GLES20.glGetUniformLocation(standardProgram, "u_Color");
    this.posHandle = GLES20.glGetAttribLocation(standardProgram, "vPosition");
    this.matrixHandle = GLES20.glGetUniformLocation(standardProgram, "uMVPMatrix");
  }

  private static int loadProgram(Context context) {
    int program = GLES20.glCreateProgram();
    int vertexShader = loadShader(GLES20.GL_VERTEX_SHADER, context, R.raw.standard_vertex_shader);
    GLES20.glAttachShader(program, vertexShader);
    int fragmentShader =
        loadShader(GLES20.GL_FRAGMENT_SHADER, context, R.raw.standard_fragment_shader);
    GLES20.glAttachShader(program, fragmentShader);
    GLES20.glLinkProgram(program);
    return program;
  }

  private static int loadShader(int type, Context context, int resourceId) {
    int shader = GLES20.glCreateShader(type);
    GLES20.glShaderSource(shader, loadResource(context, resourceId));
    GLES20.glCompileShader(shader);

    int[] compiled = new int[1];
    GLES20.glGetShaderiv(shader, GLES20.GL_COMPILE_STATUS, compiled, 0);
    if (compiled[0] == 0) {
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
    return ByteBuffer.allocateDirect(size * COORDS_PER_VERTEX * FLOAT_BYTES)
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

  public void convertToBubbleSpacePoint(Point point) {
    float glX = (2 * point.x / width) - 1;
    float glY = -((2 * point.y / height) - 1);
    point.set(glX / transformMatrix[0], glY / transformMatrix[3]);
  }

  public void drawLine(Polyline line, Color color) {
    GLES20.glEnable(GLES20.GL_BLEND);
    GLES20.glBlendFunc(GLES20.GL_SRC_ALPHA, GLES20.GL_ONE_MINUS_SRC_ALPHA);
    drawStandard(line.getVertices(), color, GLES20.GL_TRIANGLE_STRIP, 0, 0);
    drawStandard(line.getStartCap(), color, GLES20.GL_TRIANGLE_FAN, 0, 0);
    drawStandard(line.getEndCap(), color, GLES20.GL_TRIANGLE_FAN, 0, 0);
    GLES20.glDisable(GLES20.GL_BLEND);
  }

  public void drawString(String value, Color color, float x, float y) {}

  public void drawFill(FloatBuffer buffer, Color color) {
    drawStandard(buffer, color, GLES20.GL_TRIANGLE_FAN, 0, 0);
  }

  public void draw(FloatBuffer buffer, Color color, float width) {
    GLES20.glLineWidth(width);
    drawStandard(buffer, color, GLES20.GL_LINE_LOOP, 1, 1);
  }

  private void drawStandard(FloatBuffer buffer, Color color, int mode, int start, int endClip) {
    GLES20.glUseProgram(standardProgram);
    GLES20.glUniform4fv(colorHandle, 1, color.value, 0);
    GLES20.glUniformMatrix2fv(matrixHandle, 1, false, transformMatrix, 0);
    GLES20.glEnableVertexAttribArray(posHandle);
    GLES20.glVertexAttribPointer(posHandle, COORDS_PER_VERTEX, GLES20.GL_FLOAT, false, 0, buffer);
    GLES20.glDrawArrays(mode, start, buffer.limit() / COORDS_PER_VERTEX - endClip);
    GLES20.glDisableVertexAttribArray(posHandle);
  }
}
