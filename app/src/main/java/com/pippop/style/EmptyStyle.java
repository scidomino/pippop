package com.pippop.style;

import com.pippop.graphics.Color;
import com.pippop.graphics.Graphics;
import com.pippop.graphics.Polygon;

public class EmptyStyle implements Style {

  @Override
  public double getTargetArea() {
    return 0;
  }

  @Override
  public void render(Graphics g, Polygon polygon, Color outlineColor) {}
}
