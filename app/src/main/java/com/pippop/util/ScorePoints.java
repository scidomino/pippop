package com.pippop.util;

import android.content.Context;
import android.opengl.GLES20;

import com.pippop.R;
import com.pippop.programs.TextureShaderProgram;

import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.nio.FloatBuffer;
import java.util.Arrays;
import java.util.List;
import java.util.stream.IntStream;

import static android.opengl.GLES20.GL_FLOAT;
import static android.opengl.GLES20.GL_TRIANGLE_FAN;
import static android.opengl.GLES20.glDrawArrays;
import static com.pippop.Constants.BYTES_PER_FLOAT;
import static java.util.stream.Collectors.toList;

/** Created by L on 10/6/2016. */
public class ScorePoints {
  private static final int[] NUMBERS = {
    R.drawable.zero,
    R.drawable.one,
    R.drawable.two,
    R.drawable.three,
    R.drawable.four,
    R.drawable.five,
    R.drawable.six,
    R.drawable.seven,
    R.drawable.eight,
    R.drawable.nine
  };

  private static List<FloatBuffer> allMatrices =
      IntStream.range(0, 20)
          .mapToObj(
              t -> {
                float v = t * 0.06f + -0.585f;
                float v1 = t * 0.06f + -0.55f;
                float v2 = t * 0.06f + -0.525f;
                return new float[] {
                  v1, 0.75f, 0.5f, 0.5f, //
                  v, 0.65f, 0f, 1f, //
                  v2, 0.65f, 1f, 1f, //
                  v2, 0.85f, 1f, 0f, //
                  v, 0.85f, 0f, 0f, //
                  v, 0.65f, 0f, 1f
                };
              })
          .map(
              vertexData ->
                  ByteBuffer.allocateDirect(24 * BYTES_PER_FLOAT)
                      .order(ByteOrder.nativeOrder())
                      .asFloatBuffer()
                      .put(vertexData))
          .collect(toList());

  private int[] texIDMap;
  private TextureShaderProgram textureProgram;
  private FloatBuffer buffer;

  public void setupTextures(Context context) {
    this.texIDMap =
        Arrays.stream(NUMBERS).map(i -> TextureHelper.loadTexture(context, i)).toArray();
    this.textureProgram = new TextureShaderProgram(context);
  }

  public void drawPoints(float[] projectionMatrix, Context context, long points) {
    String strPoints = Long.toString(points);
    for (int k = 0; k < Math.min(19, strPoints.length()); k++) {
      int textureID = texIDMap[Character.getNumericValue(strPoints.charAt(k))];
      drawDigit(textureID, allMatrices.get(k), projectionMatrix);
    }
  }

  private void drawDigit(int textureID, FloatBuffer buffer, float[] projectionMatrix) {
    int POSITION_COMPONENT_COUNT = 2;
    int TEXTURE_COORDINATES_COMPONENT_COUNT = 2;
    int STRIDE = (POSITION_COMPONENT_COUNT + TEXTURE_COORDINATES_COMPONENT_COUNT) * BYTES_PER_FLOAT;

    textureProgram.useProgram();
    textureProgram.setUniforms(projectionMatrix, textureID);

    buffer.position(0);
    GLES20.glVertexAttribPointer(
        textureProgram.getPositionAttributeLocation(),
        POSITION_COMPONENT_COUNT,
        GL_FLOAT,
        false,
        STRIDE,
        buffer);
    GLES20.glEnableVertexAttribArray(textureProgram.getPositionAttributeLocation());

    buffer.position(POSITION_COMPONENT_COUNT);
    GLES20.glVertexAttribPointer(
        textureProgram.getTextureCoordinatesAttributeLocation(),
        TEXTURE_COORDINATES_COMPONENT_COUNT,
        GL_FLOAT,
        false,
        STRIDE,
        buffer);
    GLES20.glEnableVertexAttribArray(textureProgram.getTextureCoordinatesAttributeLocation());

    glDrawArrays(GL_TRIANGLE_FAN, 0, 6);
    GLES20.glDisableVertexAttribArray(textureProgram.getPositionAttributeLocation());
  }
}
