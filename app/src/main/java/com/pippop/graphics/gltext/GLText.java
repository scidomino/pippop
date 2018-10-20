// This is a OpenGL ES 1.0 dynamic font rendering system. It loads actual font
// files, generates a font map (texture) from them, and allows rendering of
// text strings.
//
// NOTE: the rendering portions of this class uses a sprite batcher in order
// provide decent speed rendering. Also, rendering assumes a BOTTOM-LEFT
// origin, and the (x,y) positions are relative to that, as well as the
// bottom-left of the string to render.

package com.pippop.graphics.gltext;

import android.content.res.Resources;
import android.graphics.Bitmap;
import android.graphics.Canvas;
import android.graphics.Paint;
import android.opengl.GLES20;
import android.opengl.Matrix;
import com.pippop.graphics.Color;

public class GLText {

  static final int CHAR_BATCH_SIZE = 24;
  private static final int CHAR_START = 32; // First Character (ASCII Code)
  private static final int CHAR_END = 126; // Last Character (ASCII Code)
  private static final int CHAR_CNT = CHAR_END - CHAR_START + 1;
  private static final int FONT_SIZE_MIN = 6;
  private static final int FONT_SIZE_MAX = 180;

  private final float[] charWidths;
  private final int fontPadX;
  private final int fontPadY;
  private final SpriteBatch batch;
  private final int textureId;
  private final int textureSize;
  private final TextureRegion textureRgn;
  private final float charHeight;
  private final TextureRegion[] charRgn;
  private final int cellWidth, cellHeight;
  private final int mProgram;
  private final int mColorHandle;
  private final int mTextureUniformHandle;

  private float scaleX = 1.0f;
  private float scaleY = 1.0f;
  private float spaceX = 0.0f; // Additional (X,Y Axis) Spacing (Unscaled)

  // --Constructor--//
  // D: save program + asset manager, create arrays, and initialize the members
  public GLText(
      Resources resources, int font, int size, int fontPadX, int fontPadY, boolean outline) {
    mProgram = BatchTextProgram.getProgram();
    mColorHandle = GLES20.glGetUniformLocation(mProgram, "u_Color");
    mTextureUniformHandle = GLES20.glGetUniformLocation(mProgram, "u_Texture");

    batch = new SpriteBatch(CHAR_BATCH_SIZE, mProgram);

    charWidths = new float[CHAR_CNT]; // Create the Array of Character Widths
    charRgn = new TextureRegion[CHAR_CNT]; // Create the Array of Character Regions

    this.fontPadX = fontPadX;
    this.fontPadY = fontPadY;

    Paint paint = new Paint();
    paint.setAntiAlias(true);
    paint.setTextSize(size);
    paint.setColor(0xffffffff);
    paint.setTypeface(resources.getFont(font));
    if (outline) {
      paint.setStyle(Paint.Style.STROKE);
      paint.setStrokeWidth(1);
    }

    float charWidthMax = 0;
    float[] w = new float[2];
    for (char c = CHAR_START; c <= CHAR_END; c++) {
      paint.getTextWidths(String.valueOf(c), w);
      charWidths[c - CHAR_START] = w[0];
      charWidthMax = Math.max(charWidthMax, w[0]);
    }

    Paint.FontMetrics fm = paint.getFontMetrics();
    float fontDescent = (float) Math.ceil(Math.abs(fm.descent));

    // set character height to font height
    charHeight = (float) Math.ceil(Math.abs(fm.bottom) + Math.abs(fm.top)); // Set Character Height

    // find the maximum size, validate, and setup cell sizes
    cellWidth = (int) charWidthMax + (2 * this.fontPadX);
    cellHeight = (int) charHeight + (2 * this.fontPadY);
    int maxSize = cellWidth > cellHeight ? cellWidth : cellHeight;
    if (maxSize < FONT_SIZE_MIN || maxSize > FONT_SIZE_MAX) {
      throw new RuntimeException("Font size is outside range");
    }

    if (maxSize <= 24) {
      textureSize = 256;
    } else if (maxSize <= 40) {
      textureSize = 512;
    } else if (maxSize <= 80) {
      textureSize = 1024;
    } else {
      textureSize = 2048;
    }

    // create an empty bitmap (alpha only)
    Bitmap bitmap = Bitmap.createBitmap(textureSize, textureSize, Bitmap.Config.ALPHA_8);
    bitmap.eraseColor(0x00000000); // Set Transparent Background (ARGB)

    Canvas canvas = new Canvas(bitmap);
    float x = 0;
    float y = 0;
    for (char c = CHAR_START; c <= CHAR_END; c++) {
      canvas.drawText(
          String.valueOf(c),
          x + this.fontPadX,
          y + (cellHeight - 1) - fontDescent - this.fontPadY,
          paint);
      charRgn[c - CHAR_START] =
          new TextureRegion(textureSize, textureSize, x, y, cellWidth - 1, cellHeight - 1);
      x += cellWidth; // Move to Next Character
      if ((x + cellWidth) > textureSize) {
        x = 0;
        y += cellHeight;
      }
    }

    textureId = TextureHelper.loadTexture(bitmap);

    // create full texture region
    textureRgn = new TextureRegion(textureSize, textureSize, 0, 0, textureSize, textureSize);
  }

  // --Begin/End Text Drawing--//
  // D: call these methods before/after (respectively all draw() calls using a text instance
  //    NOTE: color is set on a per-batch basis, and fonts should be 8-bit alpha only!!!
  // A: red, green, blue - RGB values for font (default = 1.0)
  //    alpha - optional alpha value for font (default = 1.0)
  // 	  vpMatrix - View and projection matrix to use
  // R: [none]
  public void begin(Color color, float[] vpMatrix) {
    initDraw(color);
    batch.beginBatch(vpMatrix); // Begin Batch
  }

  private void initDraw(Color color) {
    GLES20.glUseProgram(mProgram);

    GLES20.glUniform4fv(mColorHandle, 1, color.value, 0);
    GLES20.glEnableVertexAttribArray(mColorHandle);

    GLES20.glActiveTexture(GLES20.GL_TEXTURE0); // Set the active texture unit to texture unit 0

    GLES20.glBindTexture(GLES20.GL_TEXTURE_2D, textureId); // Bind the texture to this unit

    // Tell the texture uniform sampler to use this texture in the shader by binding to texture unit
    // 0
    GLES20.glUniform1i(mTextureUniformHandle, 0);
  }

  public void end() {
    batch.endBatch(); // End Batch
    GLES20.glDisableVertexAttribArray(mColorHandle);
  }

  // --Draw Text--//
  // D: draw text at the specified x,y position
  // A: text - the string to draw
  //    x, y, z - the x, y, z position to draw text at (bottom left of text; including descent)
  //    angleDeg - angle to rotate the text
  // R: [none]
  public void draw(String text, float x, float y, float angle) {
    float chrHeight = cellHeight * scaleY; // Calculate Scaled Character Height
    float chrWidth = cellWidth * scaleX; // Calculate Scaled Character Width
    x += (chrWidth / 2.0f) - (fontPadX * scaleX); // Adjust Start X
    y += (chrHeight / 2.0f) - (fontPadY * scaleY); // Adjust Start Y

    // create a model matrix based on x, y and angleDeg
    float[] modelMatrix = new float[16];
    Matrix.setIdentityM(modelMatrix, 0);
    Matrix.translateM(modelMatrix, 0, x, y, 1);
    Matrix.rotateM(modelMatrix, 0, angle, 0, 0, 1);

    float letterX = 0;
    for (int i = 0; i < text.length(); i++) { // FOR Each Character in String
      int c = (int) text.charAt(i) - CHAR_START;
      if (c < 0 || c >= CHAR_CNT) {
        throw new RuntimeException("Unknown Character: " + text.charAt(i));
      }
      // TODO: optimize - applying the same model matrix to all the characters in the string
      batch.drawSprite(letterX, 0, chrWidth, chrHeight, charRgn[c], modelMatrix);
      letterX += (charWidths[c] + spaceX) * scaleX;
    }
  }

  public void draw(String text, float x, float y) {
    draw(text, x, y, 0);
  }

  // --Draw Text Centered--//
  // D: draw text CENTERED at the specified x,y position
  // A: text - the string to draw
  //    x, y, z - the x, y, z position to draw text at (bottom left of text)
  //    angleDeg - angle to rotate the text
  // R: the total width of the text that was drawn
  public float drawC(String text, float x, float y, float angle) {
    float len = getLength(text); // Get Text Length
    draw(text, x - (len / 2.0f), y - (getCharHeight() / 2.0f), angle); // Draw Text Centered
    return len; // Return Length
  }

  public float drawC(String text, float x, float y) {
    return drawC(text, x, y, 0f);
  }

  public float drawCX(String text, float x, float y) {
    float len = getLength(text); // Get Text Length
    draw(text, x - (len / 2.0f), y); // Draw Text Centered (X-Axis Only)
    return len; // Return Length
  }

  public void drawCY(String text, float x, float y) {
    draw(text, x, y - (getCharHeight() / 2.0f)); // Draw Text Centered (Y-Axis Only)
  }

  // --Get Length of a String--//
  // D: return the length of the specified string if rendered using current settings
  // A: text - the string to get length for
  // R: the length of the specified string (pixels)
  private float getLength(String text) {
    float len = 0.0f; // Working Length
    for (int i = 0; i < text.length(); i++) { // For Each Character in String (Except Last
      len += getCharWidth(text.charAt(i));
    }
    len += (text.length() > 1 ? ((text.length() - 1) * spaceX) * scaleX : 0); // Add Space Length
    return len;
  }

  private float getCharWidth(char chr) {
    return (charWidths[chr - CHAR_START] * scaleX);
  }

  private float getCharHeight() {
    return (charHeight * scaleY); // Return Scaled Character Height
  }

  // --Draw Font Texture--//
  // D: draw the entire font texture (NOTE: for testing purposes only)
  // A: width, height - the width and height of the area to draw to. this is used
  //    to draw the texture to the top-left corner.
  //    vpMatrix - View and projection matrix to use
  public void drawTexture(int width, int height, float[] vpMatrix) {
    initDraw(Color.WHITE);

    batch.beginBatch(vpMatrix); // Begin Batch (Bind Texture)
    float[] idMatrix = new float[16];
    Matrix.setIdentityM(idMatrix, 0);
    batch.drawSprite(
        width - (textureSize / 2),
        height - (textureSize / 2),
        textureSize,
        textureSize,
        textureRgn,
        idMatrix); // Draw
    batch.endBatch(); // End Batch
  }
}
