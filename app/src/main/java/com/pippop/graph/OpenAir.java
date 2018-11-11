package com.pippop.graph;

import com.pippop.graphics.Graphics;
import com.pippop.style.EmptyStyle;

/** User: Tommaso Sciortino Date: Oct 24, 2011 Time: 8:49:28 PM */
public class OpenAir extends Bubble {

  public OpenAir(Edge start) {
    super(new EmptyStyle(0), start, Graphics.createFloatBuffer(1000));
  }

  public double getPressureRatio() {
    return 1;
  }

}
