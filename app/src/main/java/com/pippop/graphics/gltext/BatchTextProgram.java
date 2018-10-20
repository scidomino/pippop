package com.pippop.graphics.gltext;

import android.opengl.GLES20;

class BatchTextProgram {

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

  static int getProgram() {
    int vertexShaderHandle = loadShader(GLES20.GL_VERTEX_SHADER, vertexShaderCode);
    int fragmentShaderHandle = loadShader(GLES20.GL_FRAGMENT_SHADER, fragmentShaderCode);
    return createProgram(vertexShaderHandle, fragmentShaderHandle, programVariables);
  }

  private static int createProgram(
      int vertexShaderHandle, int fragmentShaderHandle, AttribVariable[] variables) {
    int mProgram = GLES20.glCreateProgram();

    if (mProgram == 0) {
      throw new RuntimeException("Error creating program.");
    }

    GLES20.glAttachShader(mProgram, vertexShaderHandle);
    GLES20.glAttachShader(mProgram, fragmentShaderHandle);

    for (AttribVariable var : variables) {
      GLES20.glBindAttribLocation(mProgram, var.getHandle(), var.getName());
    }

    GLES20.glLinkProgram(mProgram);

    final int[] linkStatus = new int[1];
    GLES20.glGetProgramiv(mProgram, GLES20.GL_LINK_STATUS, linkStatus, 0);
    if (linkStatus[0] == 0) {
      GLES20.glDeleteProgram(mProgram);
      throw new RuntimeException("Error creating program.");
    }
    return mProgram;
  }

  private static int loadShader(int type, String shaderCode) {
    int shaderHandle = GLES20.glCreateShader(type);
    if (shaderHandle == 0) {
      throw new RuntimeException("Error creating shader " + type);
    }

    GLES20.glShaderSource(shaderHandle, shaderCode);
    GLES20.glCompileShader(shaderHandle);

    // Get the compilation status.
    final int[] compileStatus = new int[1];
    GLES20.glGetShaderiv(shaderHandle, GLES20.GL_COMPILE_STATUS, compileStatus, 0);
    if (compileStatus[0] == 0) {
      GLES20.glDeleteShader(shaderHandle);
      throw new RuntimeException("Error creating shader " + type);
    }
    return shaderHandle;
  }
}
