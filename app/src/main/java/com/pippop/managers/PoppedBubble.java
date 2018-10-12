package com.pippop.managers;

import com.pippop.graph.Point;
import com.pippop.style.GameStyle;

public class PoppedBubble {

  private final Point center;
  private final GameStyle style;

  public PoppedBubble(Point center, GameStyle style) {
    this.center = center;
    this.style = style;
  }

  public GameStyle getStyle() {
    return style;
  }

  public Point getCenter() {
    return center;
  }
}
