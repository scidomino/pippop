package com.pippop.style;

import com.pippop.graphics.Color;
import com.pippop.graphics.Graphics;
import java.nio.FloatBuffer;

public class GameStyle implements Style {

  private final int size;
  private final int maxSize;
  private final Color color;

  public GameStyle(int size, Color color) {
    this(size, 5, color);
  }

  private GameStyle(int size, int maxSize, Color color) {
    this.size = size;
    this.maxSize = maxSize;
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

  public boolean isPoppable() {
    return size >= maxSize;
  }

  public int getPoint() {
    return size;
  }

  public Style combine(GameStyle o) {
    int newSize = size + o.size;
    int newMaxSize = Math.max(maxSize, o.maxSize);
    return new GameStyle(newSize, newMaxSize, color);
  }
}
