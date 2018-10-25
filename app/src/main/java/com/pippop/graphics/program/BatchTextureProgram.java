package com.pippop.graphics.program;

import android.content.Context;
import android.opengl.GLES20;
import com.pippop.R;
import com.pippop.graphics.Color;
import java.nio.IntBuffer;
import java.nio.ShortBuffer;

public class BatchTextureProgram extends GraphicsProgram {

  private static final int POSITION_CNT_2D = 2; // Number of Components in Vertex Color
  private static final int TEXCOORD_CNT = 2; // Number of Components in Vertex Normal
  private static final int MVP_MATRIX_INDEX_CNT = 1; // Number of Components in MVP matrix index
  private static final int VERTEX_SIZE =
      (POSITION_CNT_2D + TEXCOORD_CNT + MVP_MATRIX_INDEX_CNT) * 4;
  private final int textureCoorsHandle;
  private final int posHandle;
  private final int matrixHandle;
  private final int colorHandle;
  private final int textureHandle;

  public BatchTextureProgram(Context context) {
    super(context, R.raw.batch_texture_fragment_shader, R.raw.batch_texture_vertex_shader);
    this.textureCoorsHandle = GLES20.glGetAttribLocation(program, "a_TexCoordinate");
    this.posHandle = GLES20.glGetAttribLocation(program, "a_Position");
    this.matrixHandle = GLES20.glGetUniformLocation(program, "a_MVPMatrixIndex");
    this.colorHandle = GLES20.glGetUniformLocation(program, "u_Color");
    this.textureHandle = GLES20.glGetUniformLocation(program, "u_Texture");
  }

  public void draw(
      IntBuffer vertices, ShortBuffer indices, int numVertices, Color color, int textureId) {
    GLES20.glUseProgram(program);
    GLES20.glUniform4fv(colorHandle, 1, color.value, 0);
    GLES20.glEnableVertexAttribArray(colorHandle);
    GLES20.glActiveTexture(GLES20.GL_TEXTURE0); // Set the active texture unit to texture unit 0
    GLES20.glBindTexture(GLES20.GL_TEXTURE_2D, textureId); // Bind the texture to this unit
    // Tell the texture sampler to use this texture in the shader by binding to texture unit 0
    GLES20.glUniform1i(textureHandle, 0);

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
        textureCoorsHandle, TEXCOORD_CNT, GLES20.GL_FLOAT, false, VERTEX_SIZE, vertices);
    GLES20.glEnableVertexAttribArray(textureCoorsHandle);

    // bind MVP Matrix index position handle
    vertices.position(POSITION_CNT_2D + TEXCOORD_CNT);
    GLES20.glVertexAttribPointer(
        matrixHandle, MVP_MATRIX_INDEX_CNT, GLES20.GL_FLOAT, false, VERTEX_SIZE, vertices);
    GLES20.glEnableVertexAttribArray(matrixHandle);

    indices.position(0);
    GLES20.glDrawElements(GLES20.GL_TRIANGLES, numVertices, GLES20.GL_UNSIGNED_SHORT, indices);
    GLES20.glDisableVertexAttribArray(textureCoorsHandle);
  }
}
