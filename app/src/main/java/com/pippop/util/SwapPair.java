package com.pippop.util;

import com.pippop.graph.Bubble;
import com.pippop.graph.Edge;
import com.pippop.graph.Point;
import com.pippop.graphics.Color;
import com.pippop.graphics.Graphics;
import com.pippop.style.EmptyStyle;
import com.pippop.style.Style;
import java.nio.FloatBuffer;

public class SwapPair {

  private static final float SWAP_TIME = 200f;

  private final FloatBuffer morphedStart = Graphics.createVertexBuffer(100);
  private final FloatBuffer morphedEnd = Graphics.createVertexBuffer(100);
  private final FloatBuffer morphShape = Graphics.createVertexBuffer(100);

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
    renderSwapSide(g, center, this.top, this.bottom, topStyle);
    renderSwapSide(g, center, this.bottom, this.top, bottomStyle);
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

  private void renderSwapSide(Graphics g, Point center, Bubble startBubble, Bubble endBubble,
      Style style) {
    rotate(center, Math.PI * rotation, startBubble.getBuffer(), morphedStart);
    rotate(center, Math.PI * (rotation - 1), endBubble.getBuffer(), morphedEnd);
    Tweener.tween(morphedStart, morphedEnd, morphShape, rotation);
    style.render(g, morphShape, Color.WHITE);
  }

  public void rotate(Point center, double angle, FloatBuffer in, FloatBuffer out) {
    float sin = (float) Math.sin(angle);
    float cos = (float) Math.cos(angle);

    out.clear();
    for (int i = 0; i < in.limit() / 2; i++) {
      int xIndex = i * 2;
      int yIndex = xIndex + 1;
      float x = in.get(xIndex);
      float y = in.get(yIndex);
      float rotatedX = cos * (x - center.x) - sin * (y - center.y) + center.x;
      float rotatedY = sin * (x - center.x) + cos * (y - center.y) + center.y;

      out.put(rotatedX);
      out.put(rotatedY);
    }
    out.flip();
  }

  private class WaitingStyle extends EmptyStyle {

    private final double startTargetArea;
    private final double endTargetArea;

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
