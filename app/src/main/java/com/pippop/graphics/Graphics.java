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
import com.pippop.graphics.program.TextureProgram;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.nio.FloatBuffer;

public class Graphics {
  private static final int FLOAT_BYTES = 4;
  private static final int VIRTUAL_WIDTH = 300;

  private final StandardProgram standardProgram;
  private final GlowProgram glowProgram;

  private final float[] transformMatrix = new float[4];
  private final GLText glText;
  private final GLText glTextOutline;
  private final TextureProgram textureProgram;

  private float width;
  private float height;

  public Graphics(Context context) {
    this.standardProgram = new StandardProgram(context);
    this.glowProgram = new GlowProgram(context);
    this.textureProgram = new TextureProgram(context);

    BatchTextureProgram batchTextureProgram = new BatchTextureProgram(context);
    Typeface font = ResourcesCompat.getFont(context, R.font.sniglet_extrabold);
    glText = new GLText(batchTextureProgram, font, 30, 2, 2, false);
    glTextOutline = new GLText(batchTextureProgram, font, 30, 2, 2, true);
  }

  public static FloatBuffer createFloatBuffer(int size) {
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

  public void drawTexture(FloatBuffer buffer, int textureId) {

    textureProgram.draw(buffer, textureId, transformMatrix);
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

    glText.drawCentered(value, fillColor, x, y, mVPMatrix, 0f);
    glTextOutline.drawCentered(value, outineColor, x, y, mVPMatrix, 0f);

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
