package com.pippop.graphics.program;

import android.content.Context;
import android.opengl.GLES20;
import java.io.InputStream;
import java.util.Scanner;

public class GraphicsProgram {

  protected final int program;

  GraphicsProgram(Context context, int fragmentShader, int vertexShader) {
    this.program = GLES20.glCreateProgram();
    GLES20.glAttachShader(program, loadShader(GLES20.GL_VERTEX_SHADER, context, vertexShader));
    GLES20.glAttachShader(program, loadShader(GLES20.GL_FRAGMENT_SHADER, context, fragmentShader));
    GLES20.glLinkProgram(program);

    final int[] status = new int[1];
    GLES20.glGetProgramiv(program, GLES20.GL_LINK_STATUS, status, 0);
    if (status[0] == 0) {
      throw new RuntimeException("Error creating program: " + GLES20.glGetProgramInfoLog(program));
    }
  }

  private static int loadShader(int type, Context context, int resourceId) {
    int shader = GLES20.glCreateShader(type);
    GLES20.glShaderSource(shader, loadResource(context, resourceId));
    GLES20.glCompileShader(shader);

    int[] status = new int[1];
    GLES20.glGetShaderiv(shader, GLES20.GL_COMPILE_STATUS, status, 0);
    if (status[0] == 0) {
      String log = GLES20.glGetShaderInfoLog(shader);
      throw new RuntimeException("Could not compile shader : " + log);
    }
    return shader;
  }

  private static String loadResource(Context context, int resourceId) {
    InputStream stream = context.getResources().openRawResource(resourceId);
    return new Scanner(stream, "UTF-8").useDelimiter("\\A").next();
  }
}
