package com.pippop.graphics.gltext;

import android.opengl.GLES20;
import android.opengl.Matrix;
import com.pippop.graphics.Color;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.nio.FloatBuffer;
import java.nio.ShortBuffer;

class SpriteBatch {

  private static final int POSITION_CNT_2D = 2;
  private static final int TEXCOORD_CNT = 2;
  private static final int MVP_MATRIX_INDEX_CNT = 1;
  private static final int VERTEX_SIZE =
      (POSITION_CNT_2D + TEXCOORD_CNT + MVP_MATRIX_INDEX_CNT) * 4;
  private static final int VERTICES_PER_SPRITE = 4;
  private static final int INDICES_PER_SPRITE = 6;
  private static final int INDEX_SIZE = Short.SIZE / 8;

  private final FloatBuffer vertices;
  private final ShortBuffer indices;
  private final int maxSprites;

  private final int program;
  private final int textureCoorHandle;
  private final int posHandle;
  private final int matrixIndexHandle;
  private final int matrixHandle;
  private final int colorHandle;
  private final int textureHandle;

  private final float[] uMVPMatrices;
  private int numSprites;

  SpriteBatch(int maxSprites, int program) {
    this.maxSprites = maxSprites;
    this.numSprites = 0;
    this.uMVPMatrices = new float[maxSprites * 16];

    this.vertices =
        ByteBuffer.allocateDirect(maxSprites * VERTICES_PER_SPRITE * 5 * 4)
            .order(ByteOrder.nativeOrder())
            .asFloatBuffer();
    indices = getIndices(maxSprites);

    this.program = program;
    matrixHandle = GLES20.glGetUniformLocation(program, "u_MVPMatrix");
    textureCoorHandle = AttribVariable.A_TexCoordinate.getHandle();
    matrixIndexHandle = AttribVariable.A_MVPMatrixIndex.getHandle();
    posHandle = AttribVariable.A_Position.getHandle();
    colorHandle = GLES20.glGetUniformLocation(program, "u_Color");
    textureHandle = GLES20.glGetUniformLocation(program, "u_Texture");
  }

  private ShortBuffer getIndices(int maxSprites) {
    ShortBuffer indices =
        ByteBuffer.allocateDirect(maxSprites * INDICES_PER_SPRITE * INDEX_SIZE)
            .order(ByteOrder.nativeOrder())
            .asShortBuffer();
    for (int i = 0; i < maxSprites * VERTICES_PER_SPRITE; i += VERTICES_PER_SPRITE) {
      indices.put((short) i);
      indices.put((short) (i + 1));
      indices.put((short) (i + 2));
      indices.put((short) (i + 2));
      indices.put((short) (i + 3));
      indices.put((short) i);
    }
    indices.flip();
    return indices;
  }

  void beginBatch(Color color, int textureId) {
    numSprites = 0;

    GLES20.glUseProgram(program);
    GLES20.glUniform4fv(colorHandle, 1, color.value, 0);
    GLES20.glEnableVertexAttribArray(colorHandle);
    GLES20.glActiveTexture(GLES20.GL_TEXTURE0);
    GLES20.glBindTexture(GLES20.GL_TEXTURE_2D, textureId);
    GLES20.glUniform1i(textureHandle, 0);
  }

  void endBatch() {
    if (numSprites > 0) {
      vertices.flip();

      GLES20.glUniformMatrix4fv(matrixHandle, numSprites, false, uMVPMatrices, 0);
      GLES20.glEnableVertexAttribArray(matrixHandle);

      GLES20.glVertexAttribPointer(
          posHandle, POSITION_CNT_2D, GLES20.GL_FLOAT, false, VERTEX_SIZE, vertices);
      GLES20.glEnableVertexAttribArray(posHandle);

      vertices.position(POSITION_CNT_2D);
      GLES20.glVertexAttribPointer(
          textureCoorHandle, TEXCOORD_CNT, GLES20.GL_FLOAT, false, VERTEX_SIZE, vertices);
      GLES20.glEnableVertexAttribArray(textureCoorHandle);

      vertices.position(POSITION_CNT_2D + TEXCOORD_CNT);
      GLES20.glVertexAttribPointer(
          matrixIndexHandle, MVP_MATRIX_INDEX_CNT, GLES20.GL_FLOAT, false, VERTEX_SIZE, vertices);
      GLES20.glEnableVertexAttribArray(matrixIndexHandle);

      GLES20.glDrawElements(
          GLES20.GL_TRIANGLES, numSprites * INDICES_PER_SPRITE, GLES20.GL_UNSIGNED_SHORT, indices);

      GLES20.glDisableVertexAttribArray(textureCoorHandle);

      vertices.clear();
    }
    GLES20.glDisableVertexAttribArray(colorHandle);
  }

  void drawSprite(
      float x,
      float y,
      float width,
      float height,
      TextureRegion region,
      float[] modelMatrix,
      float[] vpMatrix) {
    if (numSprites == maxSprites) {
      endBatch();
      numSprites = 0;
    }

    float halfWidth = width / 2.0f;
    float halfHeight = height / 2.0f;
    float x1 = x - halfWidth;
    float y1 = y - halfHeight;
    float x2 = x + halfWidth;
    float y2 = y + halfHeight;

    vertices.put(x1).put(y1).put(region.u1).put(region.v2).put(numSprites);
    vertices.put(x2).put(y1).put(region.u2).put(region.v2).put(numSprites);
    vertices.put(x2).put(y2).put(region.u2).put(region.v1).put(numSprites);
    vertices.put(x1).put(y2).put(region.u1).put(region.v1).put(numSprites);

    Matrix.multiplyMM(uMVPMatrices, numSprites * 16, vpMatrix, 0, modelMatrix, 0);

    numSprites++;
  }
}
