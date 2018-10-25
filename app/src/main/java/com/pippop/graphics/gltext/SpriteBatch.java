package com.pippop.graphics.gltext;

import android.opengl.GLES20;
import android.opengl.Matrix;
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
  private final int mTextureCoordinateHandle;
  private final int mPositionHandle;
  private final int mMVPIndexHandle;
  private int maxSprites;
  private int numSprites;
  private float[] mVPMatrix;
  private float[] uMVPMatrices;
  private int mMVPMatricesHandle;
  private float[] mMVPMatrix = new float[16];

  SpriteBatch(int maxSprites, int program) {
    this.maxSprites = maxSprites;
    this.numSprites = 0;
    this.uMVPMatrices = new float[maxSprites * 16];

    this.vertices =
        ByteBuffer.allocateDirect(maxSprites * VERTICES_PER_SPRITE * 5 * 4)
            .order(ByteOrder.nativeOrder())
            .asFloatBuffer();
    indices = getIndices(maxSprites);

    mMVPMatricesHandle = GLES20.glGetUniformLocation(program, "u_MVPMatrix");
    mTextureCoordinateHandle = AttribVariable.A_TexCoordinate.getHandle();
    mMVPIndexHandle = AttribVariable.A_MVPMatrixIndex.getHandle();
    mPositionHandle = AttribVariable.A_Position.getHandle();
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

  void beginBatch(float[] vpMatrix) {
    numSprites = 0; // Empty Sprite Counter
    mVPMatrix = vpMatrix;
  }

  void endBatch() {
    if (numSprites > 0) {
      vertices.flip();

      GLES20.glUniformMatrix4fv(mMVPMatricesHandle, numSprites, false, uMVPMatrices, 0);
      GLES20.glEnableVertexAttribArray(mMVPMatricesHandle);

      GLES20.glVertexAttribPointer(
          mPositionHandle, POSITION_CNT_2D, GLES20.GL_FLOAT, false, VERTEX_SIZE, vertices);
      GLES20.glEnableVertexAttribArray(mPositionHandle);

      vertices.position(POSITION_CNT_2D);
      GLES20.glVertexAttribPointer(
          mTextureCoordinateHandle, TEXCOORD_CNT, GLES20.GL_FLOAT, false, VERTEX_SIZE, vertices);
      GLES20.glEnableVertexAttribArray(mTextureCoordinateHandle);

      vertices.position(POSITION_CNT_2D + TEXCOORD_CNT);
      GLES20.glVertexAttribPointer(
          mMVPIndexHandle, MVP_MATRIX_INDEX_CNT, GLES20.GL_FLOAT, false, VERTEX_SIZE, vertices);
      GLES20.glEnableVertexAttribArray(mMVPIndexHandle);

      GLES20.glDrawElements(
          GLES20.GL_TRIANGLES, numSprites * INDICES_PER_SPRITE, GLES20.GL_UNSIGNED_SHORT, indices);

      GLES20.glDisableVertexAttribArray(mTextureCoordinateHandle);

      vertices.clear();
    }
  }

  void drawSprite(
      float x, float y, float width, float height, TextureRegion region, float[] modelMatrix) {
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

    Matrix.multiplyMM(mMVPMatrix, 0, mVPMatrix, 0, modelMatrix, 0);
    System.arraycopy(mMVPMatrix, 0, uMVPMatrices, numSprites * 16, 16);

    numSprites++;
  }
}
