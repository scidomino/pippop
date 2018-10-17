package com.pippop.style;

import com.pippop.graphics.Color;
import com.pippop.graphics.Graphics;
import java.nio.FloatBuffer;

public class EmptyStyle implements Style {

  @Override
  public double getTargetArea() {
    return 0;
  }

  @Override
  public void render(Graphics g, FloatBuffer buffer, Color outlineColor) {
  }
}
