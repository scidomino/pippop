// This is a OpenGL ES 1.0 dynamic font rendering system. It loads actual font
// files, generates a font map (texture) from them, and allows rendering of
// text strings.
//
// NOTE: the rendering portions of this class uses a sprite batcher in order
// provide decent speed rendering. Also, rendering assumes a BOTTOM-LEFT
// origin, and the (x,y) positions are relative to that, as well as the
// bottom-left of the string to render.

package com.pippop.graphics.gltext;

import android.graphics.Bitmap;
import android.graphics.Canvas;
import android.graphics.Paint;
import android.graphics.Typeface;
import android.opengl.Matrix;
import com.pippop.graphics.Color;
import com.pippop.graphics.program.BatchTextureProgram;

public class GLText {

  private static final int CHAR_START = 32;
  private static final int CHAR_END = 126;
  private static final int CHAR_CNT = CHAR_END - CHAR_START + 1;
  private static final int FONT_SIZE_MIN = 6;
  private static final int FONT_SIZE_MAX = 180;

  private final float[] charWidths;
  private final int fontPadX;
  private final int fontPadY;
  private final SpriteBatch batch;
  private final int textureId;
  private final float charHeight;
  private final TextureRegion[] charRgn;
  private final int cellWidth, cellHeight;

  private float scaleX = 1.0f;
  private float scaleY = 1.0f;
  private float spaceX = 0.0f; // Additional (X,Y Axis) Spacing (Unscaled)

  // --Constructor--//
  // D: save program + asset manager, create arrays, and initialize the members
  public GLText(
      BatchTextureProgram program,
      Typeface font,
      int size,
      int fontPadX,
      int fontPadY,
      boolean outline) {
    batch = new SpriteBatch(24, program);

    charWidths = new float[CHAR_CNT]; // Create the Array of Character Widths
    charRgn = new TextureRegion[CHAR_CNT]; // Create the Array of Character Regions

    this.fontPadX = fontPadX;
    this.fontPadY = fontPadY;

    Paint paint = new Paint();
    paint.setAntiAlias(true);
    paint.setTextSize(size);
    paint.setColor(0xffffffff);
    paint.setTypeface(font);
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

    int textureSize;
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
  }

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

  public void drawCentered(
      String value, Color fillColor, float x, float y, float[] mVPMatrix, float angle) {
    draw(
        value,
        fillColor,
        mVPMatrix,
        angle,
        x - (getLength(value) / 2.0f),
        y - (getCharHeight() / 2.0f));
  }

  private void draw(String value, Color color, float[] mVPMatrix, float angle, float x, float y) {

    batch.beginBatch(color, textureId);

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
    for (int i = 0; i < value.length(); i++) { // FOR Each Character in String
      int c = (int) value.charAt(i) - CHAR_START;
      if (c < 0 || c >= CHAR_CNT) {
        throw new RuntimeException("Unknown Character: " + value.charAt(i));
      }
      batch.drawSprite(letterX, 0, chrWidth, chrHeight, charRgn[c], modelMatrix, mVPMatrix);
      letterX += (charWidths[c] + spaceX) * scaleX;
    }

    batch.endBatch();
  }
}
