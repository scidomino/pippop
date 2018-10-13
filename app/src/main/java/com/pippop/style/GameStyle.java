package com.pippop.style;

import com.pippop.graphics.Color;
import com.pippop.graphics.Graphics;
import com.pippop.graphics.Polygon;
import java.nio.FloatBuffer;

public class GameStyle implements Style {

  private final int size;
  private final int maxSize;
  private final Color color;

  public GameStyle(int size, Color color) {
    this(size, 5, color);
  }

  public GameStyle(int size, int maxSize, Color color) {
    this.size = size;
    this.maxSize = maxSize;
    this.color = color;
  }

  public Color getColor() {
    return color;
  }

  @Override
  public void render(Graphics g, Polygon polygon, Color outlineColor) {
    g.drawFill(polygon, this.color);
    g.draw(polygon, outlineColor, 4);
    FloatBuffer buffer = polygon.getVertices();
    g.drawString(String.valueOf(size), Color.WHITE, buffer.get(0), buffer.get(1));
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
