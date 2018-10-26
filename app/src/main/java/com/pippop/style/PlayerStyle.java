package com.pippop.style;

import android.content.Context;
import android.graphics.Bitmap;
import android.graphics.BitmapFactory;
import com.pippop.R;
import com.pippop.graphics.Color;
import com.pippop.graphics.Graphics;
import com.pippop.util.TextureHelper;
import java.nio.FloatBuffer;

public class PlayerStyle implements Style {

  private final int size = 1;
  private final Color color = Color.TRANSPARENT_WHITE;
  private final Context context;
  private Otter otter;

  public PlayerStyle(Context context) {
    this.context = context;
  }

  public Color getColor() {
    return color;
  }

  @Override
  public void render(Graphics g, FloatBuffer buffer, Color outlineColor) {
    g.drawFill(buffer, this.color);
    g.draw(buffer, outlineColor, 4);

    if (otter == null) {
      otter = new Otter(context);
    }

    g.drawTexture(otter.getBuffer(buffer.get(0), buffer.get(1)), otter.textureId);
  }

  @Override
  public double getTargetArea() {
    return 3000 * Math.sqrt(size);
  }

  public int getPoint() {
    return size;
  }

  class Otter {

    private final int textureId;
    private final FloatBuffer buffer = Graphics.createFloatBuffer(16);

    Otter(Context context) {
      Bitmap bitmap = BitmapFactory.decodeResource(context.getResources(), R.drawable.otter);
      this.textureId = TextureHelper.loadTexture(bitmap);
    }

    FloatBuffer getBuffer(float centerX, float centerY) {
      float radius = 25;
      buffer.clear();
      buffer.put(centerX - radius).put(centerY - radius).put(0).put(1);
      buffer.put(centerX + radius).put(centerY - radius).put(1).put(1);
      buffer.put(centerX + radius).put(centerY + radius).put(1).put(0);
      buffer.put(centerX - radius).put(centerY + radius).put(0).put(0);
      buffer.flip();
      return buffer;
    }
  }
}
