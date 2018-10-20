package com.pippop.graphics.gltext.programs;

import android.opengl.GLES20;
import com.pippop.graphics.gltext.AttribVariable;
import com.pippop.graphics.gltext.Utilities;

public class BatchTextProgram {

  private static final AttribVariable[] programVariables = {
      AttribVariable.A_Position, AttribVariable.A_TexCoordinate, AttribVariable.A_MVPMatrixIndex
  };

  private static final String vertexShaderCode =
      "uniform mat4 u_MVPMatrix[24];      \n"
          + "attribute float a_MVPMatrixIndex; \n"
          + "attribute vec4 a_Position;     \n"
          + "attribute vec2 a_TexCoordinate;\n"
          + "varying vec2 v_TexCoordinate;  \n"
          + "void main()                    \n"
          + "{                              \n"
          + "   v_TexCoordinate = a_TexCoordinate; \n"
          + "   gl_Position = u_MVPMatrix[int(a_MVPMatrixIndex)] * a_Position;   \n"
          + "}                              \n";

  private static final String fragmentShaderCode =
      "uniform sampler2D u_Texture;       \n"
          + "precision mediump float;       \n"
          + "uniform vec4 u_Color;          \n"
          + "varying vec2 v_TexCoordinate;  \n"
          + "void main()                    \n"
          + "{                              \n"
          + "   gl_FragColor = texture2D(u_Texture, v_TexCoordinate).w * u_Color;\n"
          + "}                             \n";

  public static int getProgram() {
    int vertexShaderHandle = Utilities.loadShader(GLES20.GL_VERTEX_SHADER, vertexShaderCode);
    int fragmentShaderHandle = Utilities.loadShader(GLES20.GL_FRAGMENT_SHADER, fragmentShaderCode);
    return Utilities.createProgram(vertexShaderHandle, fragmentShaderHandle, programVariables);
  }
}
