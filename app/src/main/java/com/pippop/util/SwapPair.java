package com.pippop.util;

import com.pippop.graph.Bubble;
import com.pippop.graph.Edge;
import com.pippop.graph.Point;
import com.pippop.graphics.Color;
import com.pippop.graphics.Graphics;
import com.pippop.graphics.Polygon;
import com.pippop.style.EmptyStyle;
import com.pippop.style.Style;

public class SwapPair {

  private static final float SWAP_TIME = 200f;

  private final Polygon morphedStart = new Polygon(100);
  private final Polygon morphedEnd = new Polygon(100);
  private final MorphShape morphShape = new MorphShape();

  private final Edge edge;
  private final Bubble top;
  private final Style topStyle;
  private final Bubble bottom;
  private final Style bottomStyle;
  private final boolean returnTrip;

  // 0 is no rotation, 1 corresponds to 180 degrees
  private float rotation;

  public SwapPair(Edge edge, boolean returnTrip) {
    this.edge = edge;
    this.returnTrip = returnTrip;

    this.edge.makeFirstEdge();
    this.edge.getTwin().makeFirstEdge();

    this.top = edge.getTwin().getBubble();
    this.topStyle = top.getStyle();

    this.bottom = edge.getBubble();
    this.bottomStyle = bottom.getStyle();

    this.top.setStyle(new WaitingStyle(topStyle.getTargetArea(), bottomStyle.getTargetArea()));
    this.bottom.setStyle(new WaitingStyle(bottomStyle.getTargetArea(), topStyle.getTargetArea()));
  }

  public Edge getEdge() {
    return edge;
  }

  public boolean isReturnTrip() {
    return this.returnTrip;
  }

  public void switchBubbleProps() {
    top.setStyle(bottomStyle);
    bottom.setStyle(topStyle);
  }

  public void draw(Graphics g) {
    Point center = this.edge.getCenter();
    tween(g, center, this.top, this.bottom, topStyle);
    tween(g, center, this.bottom, this.top, bottomStyle);
  }

  public void move(int delta) {
    this.rotation += delta / SWAP_TIME;
  }

  public boolean isDone() {
    return this.rotation >= 1;
  }

  public Bubble getTop() {
    return this.top;
  }

  public Bubble getBottom() {
    return this.bottom;
  }

  public void tween(Graphics g, Point center, Bubble startBubble, Bubble endBubble, Style style) {
    morphedStart.rotate(center, Math.PI * rotation, startBubble.getShape());
    morphedEnd.rotate(center, Math.PI * (rotation - 1), endBubble.getShape());
    morphShape.build(morphedStart, morphedEnd, rotation);
    style.render(g, morphShape, Color.WHITE);
  }

  private class WaitingStyle extends EmptyStyle {
    private double startTargetArea;
    private double endTargetArea;

    WaitingStyle(double startTargetArea, double endTargetArea) {
      this.startTargetArea = startTargetArea;
      this.endTargetArea = endTargetArea;
    }

    @Override
    public double getTargetArea() {
      return (1 - rotation) * startTargetArea + rotation * endTargetArea;
    }
  }
}
