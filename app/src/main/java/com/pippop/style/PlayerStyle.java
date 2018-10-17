package com.pippop.style;

import com.pippop.graphics.Color;
import com.pippop.graphics.Graphics;
import java.nio.FloatBuffer;

public class PlayerStyle implements Style {

  private final int size;
  private final Color color;

  public PlayerStyle(int size, Color color) {
    this.size = size;
    this.color = color;
  }

  public Color getColor() {
    return color;
  }

  @Override
  public void render(Graphics g, FloatBuffer buffer, Color outlineColor) {
    g.drawFill(buffer, this.color);
    g.draw(buffer, outlineColor, 4);
  }

  @Override
  public double getTargetArea() {
    return 3000 * Math.sqrt(size);
  }

  public int getPoint() {
    return size;
  }

  public Style combine(PlayerStyle o) {
    return this;
  }
}
