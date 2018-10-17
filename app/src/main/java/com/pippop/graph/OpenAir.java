package com.pippop.graph;

import com.pippop.graphics.Graphics;
import com.pippop.style.EmptyStyle;

/** User: Tommaso Sciortino Date: Oct 24, 2011 Time: 8:49:28 PM */
public class OpenAir extends Bubble {

  public OpenAir(Edge start) {
    super(new EmptyStyle(), start, Graphics.createVertexBuffer(1000));
  }

  public double getPressureRatio(double speedBump) {
    return -1;
  }

  public boolean contains(Point p) {
    return false;
  }
}
