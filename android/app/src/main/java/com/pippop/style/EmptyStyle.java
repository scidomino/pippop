package com.pippop.style;

import com.pippop.graphics.Color;
import com.pippop.graphics.Graphics;
import java.nio.FloatBuffer;

public class EmptyStyle implements Style {

  private final double targetArea;

  public EmptyStyle(double targetArea) {
    this.targetArea = targetArea;
  }

  @Override
  public double getTargetArea() {
    return targetArea;
  }

  @Override
  public void render(Graphics g, FloatBuffer buffer, Color outlineColor) {
  }
}
