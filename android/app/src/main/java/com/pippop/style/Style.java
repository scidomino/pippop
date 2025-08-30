package com.pippop.style;

import com.pippop.graphics.Color;
import com.pippop.graphics.Graphics;
import java.nio.FloatBuffer;

public interface Style {

  double getTargetArea();

  void render(Graphics graphics, FloatBuffer shape, Color outlineColor);
}
