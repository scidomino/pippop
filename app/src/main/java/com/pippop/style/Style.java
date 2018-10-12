package com.pippop.style;

import com.pippop.graphics.Color;
import com.pippop.graphics.Graphics;
import com.pippop.graphics.Polygon;

public interface Style {

  double getTargetArea();

  void render(Graphics graphics, Polygon shape, Color outlineColor);
}
