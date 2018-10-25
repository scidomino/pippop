package com.pippop.graphics.gltext;

import android.opengl.GLES20;
import java.nio.FloatBuffer;
import java.nio.ShortBuffer;

class Vertices {

  private static final int POSITION_CNT_2D = 2;
  private static final int TEXCOORD_CNT = 2;
  private static final int MVP_MATRIX_INDEX_CNT = 1;
  private static final int VERTEX_SIZE =
      (POSITION_CNT_2D + TEXCOORD_CNT + MVP_MATRIX_INDEX_CNT) * 4;

  private final ShortBuffer indices;

  private final int mTextureCoordinateHandle;
  private int mPositionHandle;
  private int mMVPIndexHandle;

  Vertices(ShortBuffer indices) {
    this.indices = indices;

    mTextureCoordinateHandle = AttribVariable.A_TexCoordinate.getHandle();
    mMVPIndexHandle = AttribVariable.A_MVPMatrixIndex.getHandle();
    mPositionHandle = AttribVariable.A_Position.getHandle();
  }

  void bind(FloatBuffer vertices) {
    vertices.position(0);
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
  }

  void draw(int numVertices) {
    indices.position(0);
    GLES20.glDrawElements(GLES20.GL_TRIANGLES, numVertices, GLES20.GL_UNSIGNED_SHORT, indices);
  }

  void unbind() {
    GLES20.glDisableVertexAttribArray(mTextureCoordinateHandle);
  }
}
