package com.pippop.programs;

/** Created by L on 10/5/2016. */
import android.content.Context;

import com.pippop.R;

import java.io.InputStream;
import java.util.Scanner;

import static android.opengl.GLES20.GL_BLEND;
import static android.opengl.GLES20.GL_COMPILE_STATUS;
import static android.opengl.GLES20.GL_FRAGMENT_SHADER;
import static android.opengl.GLES20.GL_LINK_STATUS;
import static android.opengl.GLES20.GL_ONE_MINUS_SRC_ALPHA;
import static android.opengl.GLES20.GL_SRC_ALPHA;
import static android.opengl.GLES20.GL_TEXTURE0;
import static android.opengl.GLES20.GL_TEXTURE_2D;
import static android.opengl.GLES20.GL_VALIDATE_STATUS;
import static android.opengl.GLES20.GL_VERTEX_SHADER;
import static android.opengl.GLES20.glActiveTexture;
import static android.opengl.GLES20.glAttachShader;
import static android.opengl.GLES20.glBindTexture;
import static android.opengl.GLES20.glBlendFunc;
import static android.opengl.GLES20.glCompileShader;
import static android.opengl.GLES20.glCreateProgram;
import static android.opengl.GLES20.glCreateShader;
import static android.opengl.GLES20.glDeleteProgram;
import static android.opengl.GLES20.glDeleteShader;
import static android.opengl.GLES20.glEnable;
import static android.opengl.GLES20.glGetAttribLocation;
import static android.opengl.GLES20.glGetProgramInfoLog;
import static android.opengl.GLES20.glGetProgramiv;
import static android.opengl.GLES20.glGetShaderInfoLog;
import static android.opengl.GLES20.glGetShaderiv;
import static android.opengl.GLES20.glGetUniformLocation;
import static android.opengl.GLES20.glLinkProgram;
import static android.opengl.GLES20.glShaderSource;
import static android.opengl.GLES20.glUniform1i;
import static android.opengl.GLES20.glUniformMatrix4fv;
import static android.opengl.GLES20.glUseProgram;
import static android.opengl.GLES20.glValidateProgram;

public class TextureShaderProgram {

  // Uniform constants
  private static final String U_MATRIX = "u_Matrix";
  private static final String U_TEXTURE_UNIT = "u_TextureUnit";

  // Attribute constants
  private static final String A_POSITION = "a_Position";
  private static final String A_TEXTURE_COORDINATES = "a_TextureCoordinates";

  private final int program;

  private final int uMatrixLocation;
  private final int uTextureUnitLocation;

  private final int aPositionLocation;
  private final int aTextureCoordinatesLocation;

  public TextureShaderProgram(Context context) {
    program = buildProgram(context, R.raw.texture_vertex_shader, R.raw.texture_fragment_shader);

    // Retrieve uniform locations for the shader program.
    uMatrixLocation = glGetUniformLocation(program, U_MATRIX);
    uTextureUnitLocation = glGetUniformLocation(program, U_TEXTURE_UNIT);

    // Retrieve attribute locations for the shader program.
    aPositionLocation = glGetAttribLocation(program, A_POSITION);
    aTextureCoordinatesLocation = glGetAttribLocation(program, A_TEXTURE_COORDINATES);
  }

  private static int buildProgram(Context context, int vertexShaderId, int fragmentShaderId) {
    String vertexShaderCode = readTextFileFromResource(context, vertexShaderId);
    int vertexShader = compileShader(GL_VERTEX_SHADER, vertexShaderCode);

    String fragmentShaderCode = readTextFileFromResource(context, fragmentShaderId);
    int fragmentShader = compileShader(GL_FRAGMENT_SHADER, fragmentShaderCode);

    return linkProgram(vertexShader, fragmentShader);
  }

  private static int compileShader(int type, String shaderCode) {
    int shaderId = glCreateShader(type);
    if (shaderId == 0) {
      throw new RuntimeException("Could not create new shader.");
    }

    glShaderSource(shaderId, shaderCode);
    glCompileShader(shaderId);

    int[] compileStatus = new int[1];
    glGetShaderiv(shaderId, GL_COMPILE_STATUS, compileStatus, 0);
    if (compileStatus[0] == 0) {
      glDeleteShader(shaderId);
      throw new RuntimeException(glGetShaderInfoLog(shaderId));
    }
    return shaderId;
  }

  private static int linkProgram(int vertexShaderId, int fragmentShaderId) {
    int programId = glCreateProgram();
    if (programId == 0) {
      throw new RuntimeException("Could not create new program");
    }

    glAttachShader(programId, vertexShaderId);
    glAttachShader(programId, fragmentShaderId);
    glLinkProgram(programId);

    int[] linkStatus = new int[1];
    glGetProgramiv(programId, GL_LINK_STATUS, linkStatus, 0);
    if (linkStatus[0] == 0) {
      glDeleteProgram(programId);
      throw new RuntimeException(glGetProgramInfoLog(programId));
    }
    //    validateProgram(program);
    return programId;
  }

  private static String readTextFileFromResource(Context context, int resourceId) {
    InputStream inputStream = context.getResources().openRawResource(resourceId);
    Scanner s = new Scanner(inputStream).useDelimiter("\\A");
    return s.hasNext() ? s.next() : "";
  }

  /** Validates an OpenGL program. Should only be called when developing the application. */
  private static void validateProgram(int programId) {
    glValidateProgram(programId);
    final int[] validateStatus = new int[1];
    glGetProgramiv(programId, GL_VALIDATE_STATUS, validateStatus, 0);
    if (validateStatus[0] != 0) {
      throw new RuntimeException(glGetProgramInfoLog(programId));
    }
  }

  public void useProgram() {
    // Set the current OpenGL shader program to this program.
    glUseProgram(program);
  }

  public void setUniforms(float[] matrix, int textureId) {
    // Pass the matrix into the shader program.
    glUniformMatrix4fv(uMatrixLocation, 1, false, matrix, 0);

    // Set the active texture unit to texture unit 0.
    glActiveTexture(GL_TEXTURE0);

    // Bind the texture to this unit.
    glBindTexture(GL_TEXTURE_2D, textureId);
    glEnable(GL_BLEND);
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    // Tell the texture uniform sampler to use this texture in the shader by
    // telling it to read from texture unit 0.
    glUniform1i(uTextureUnitLocation, 0);
  }

  public int getPositionAttributeLocation() {
    return aPositionLocation;
  }

  public int getTextureCoordinatesAttributeLocation() {
    return aTextureCoordinatesLocation;
  }
}
