package com.pippop.graphics.gltext;

import android.opengl.GLES20;

public class Utilities {

  public static int createProgram(
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

  public static int loadShader(int type, String shaderCode) {
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
