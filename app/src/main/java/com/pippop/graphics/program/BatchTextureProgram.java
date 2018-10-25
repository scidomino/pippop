package com.pippop.graphics.program;

import android.content.Context;
import android.opengl.GLES20;
import com.pippop.R;
import java.nio.IntBuffer;
import java.nio.ShortBuffer;

public class BatchTextureProgram extends GraphicsProgram {

  private static final int POSITION_CNT_2D = 2; // Number of Components in Vertex Color
  private static final int TEXCOORD_CNT = 2; // Number of Components in Vertex Normal
  private static final int MVP_MATRIX_INDEX_CNT = 1; // Number of Components in MVP matrix index
  private static final int INDEX_SIZE = Short.SIZE / 8; // Index Byte Size (Short.SIZE = bits)
  private static final int VERTEX_SIZE =
      (POSITION_CNT_2D + TEXCOORD_CNT + MVP_MATRIX_INDEX_CNT) * 4;
  private final int textureHandle;
  private final int posHandle;
  private final int matrixHandle;

  public BatchTextureProgram(Context context) {
    super(context, R.raw.batch_texture_fragment_shader, R.raw.batch_texture_vertex_shader);
    this.textureHandle = GLES20.glGetAttribLocation(program, "a_TexCoordinate");
    this.posHandle = GLES20.glGetAttribLocation(program, "a_Position");
    this.matrixHandle = GLES20.glGetUniformLocation(program, "a_MVPMatrixIndex");
  }

  void draw(
      IntBuffer vertices, ShortBuffer indices, int primitiveType, int offset, int numVertices) {
    // bind vertex position pointer
    vertices.position(0); // Set Vertex Buffer to Position
    GLES20.glVertexAttribPointer(
        posHandle, POSITION_CNT_2D, GLES20.GL_FLOAT, false, VERTEX_SIZE, vertices);
    GLES20.glEnableVertexAttribArray(posHandle);

    // bind texture position pointer
    vertices.position(POSITION_CNT_2D);
    // color
    // is also specified)
    GLES20.glVertexAttribPointer(
        textureHandle, TEXCOORD_CNT, GLES20.GL_FLOAT, false, VERTEX_SIZE, vertices);
    GLES20.glEnableVertexAttribArray(textureHandle);

    // bind MVP Matrix index position handle
    vertices.position(POSITION_CNT_2D + TEXCOORD_CNT);
    GLES20.glVertexAttribPointer(
        matrixHandle, MVP_MATRIX_INDEX_CNT, GLES20.GL_FLOAT, false, VERTEX_SIZE, vertices);
    GLES20.glEnableVertexAttribArray(matrixHandle);

    if (indices != null) { // IF Indices Exist
      indices.position(offset); // Set Index Buffer to Specified Offset
      // draw indexed
      GLES20.glDrawElements(primitiveType, numVertices, GLES20.GL_UNSIGNED_SHORT, indices);
    } else { // ELSE No Indices Exist
      // draw direct
      GLES20.glDrawArrays(primitiveType, offset, numVertices);
    }
    GLES20.glDisableVertexAttribArray(textureHandle);
  }
}
