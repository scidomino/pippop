package com.pippop.graphics.gltext;

import android.opengl.GLES20;
import java.nio.FloatBuffer;
import java.nio.ShortBuffer;

class Vertices {

  // --Constants--//
  private static final int POSITION_CNT_2D = 2; // Number of Components in Vertex Color
  private static final int TEXCOORD_CNT = 2; // Number of Components in Vertex Normal
  private static final int MVP_MATRIX_INDEX_CNT = 1; // Number of Components in MVP matrix index
  private static final int INDEX_SIZE = Short.SIZE / 8; // Index Byte Size (Short.SIZE = bits)
  private static final int VERTEX_SIZE =
      (POSITION_CNT_2D + TEXCOORD_CNT + MVP_MATRIX_INDEX_CNT) * 4;

  private final ShortBuffer indices;

  private final int mTextureCoordinateHandle;
  private int mPositionHandle;
  private int mMVPIndexHandle;

  // --Constructor--//
  // D: create the vertices/indices as specified (for 2d/3d)
  // A: maxVertices - maximum vertices allowed in buffer
  //    maxIndices - maximum indices allowed in buffer
  Vertices(ShortBuffer indices) {
    this.indices = indices;

    mTextureCoordinateHandle = AttribVariable.A_TexCoordinate.getHandle();
    mMVPIndexHandle = AttribVariable.A_MVPMatrixIndex.getHandle();
    mPositionHandle = AttribVariable.A_Position.getHandle();
  }

  // --Bind--//
  // D: perform all required binding/state changes before rendering batches.
  //    USAGE: call once before calling draw() multiple times for this buffer.
  // A: [none]
  // R: [none]
  void bind(FloatBuffer vertices) {
    // bind vertex position pointer
    vertices.position(0); // Set Vertex Buffer to Position
    GLES20.glVertexAttribPointer(
        mPositionHandle, POSITION_CNT_2D, GLES20.GL_FLOAT, false, VERTEX_SIZE, vertices);
    GLES20.glEnableVertexAttribArray(mPositionHandle);

    // bind texture position pointer
    vertices.position(POSITION_CNT_2D);
    // color
    // is also specified)
    GLES20.glVertexAttribPointer(
        mTextureCoordinateHandle, TEXCOORD_CNT, GLES20.GL_FLOAT, false, VERTEX_SIZE, vertices);
    GLES20.glEnableVertexAttribArray(mTextureCoordinateHandle);

    // bind MVP Matrix index position handle
    vertices.position(POSITION_CNT_2D + TEXCOORD_CNT);
    GLES20.glVertexAttribPointer(
        mMVPIndexHandle, MVP_MATRIX_INDEX_CNT, GLES20.GL_FLOAT, false, VERTEX_SIZE, vertices);
    GLES20.glEnableVertexAttribArray(mMVPIndexHandle);
  }

  // --Draw--//
  // D: draw the currently bound vertices in the vertex/index buffers
  //    USAGE: can only be called after calling bind() for this buffer.
  // A: primitiveType - the type of primitive to draw
  //    offset - the offset in the vertex/index buffer to start at
  //    numVertices - the number of vertices (indices) to draw
  // R: [none]
  public void draw(int primitiveType, int offset, int numVertices) {
    if (indices != null) { // IF Indices Exist
      indices.position(offset); // Set Index Buffer to Specified Offset
      // draw indexed
      GLES20.glDrawElements(primitiveType, numVertices, GLES20.GL_UNSIGNED_SHORT, indices);
    } else { // ELSE No Indices Exist
      // draw direct
      GLES20.glDrawArrays(primitiveType, offset, numVertices);
    }
  }

  // --Unbind--//
  // D: clear binding states when done rendering batches.
  //    USAGE: call once before calling draw() multiple times for this buffer.
  // A: [none]
  // R: [none]
  void unbind() {
    GLES20.glDisableVertexAttribArray(mTextureCoordinateHandle);
  }
}
