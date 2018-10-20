package com.pippop.graph;

import java.nio.FloatBuffer;

/**
 * Represents one side of a bubble wall. Together with it's twin it describes a cubic bezier curve
 * with control points (start, startCtrl, twin.startCtrl, twin.start) sometimes referred to as
 * (start, startCtrl, endCtrl, end).
 *
 * <p>User: Tommaso Sciortino Date: Oct 16, 2011 Time: 9:57:10 AM
 */
public class Edge {

  private static final double FLATNESS = .5;
  private final Edge twin;
  private final Variable startCtrl;
  // only updated when graph is modified
  private Vertex start;
  private Bubble bubble;
  private Edge next;
  // updated every time the position moves
  private double halfCentroidComponentY;
  private double halfCentroidComponentX;
  private double halfArea;

  public Edge(Vertex start, Vertex end) {
    Variable startCtrl = makeTwoThirdsVar(start, end);
    Variable endCtrl = makeTwoThirdsVar(end, start);

    setStart(start);

    this.startCtrl = startCtrl;
    this.twin = new Edge(this, end, endCtrl);

    update();
  }

  private Edge(Edge twin, Vertex start, Variable ctrl) {
    this.twin = twin;
    setStart(start);
    this.startCtrl = ctrl;

    update();
  }

  private static Variable makeTwoThirdsVar(Vertex closer, Vertex farther) {
    float x = (2 * closer.x + farther.x) / 3;
    float y = (2 * closer.y + farther.y) / 3;
    return new Variable(x, y);
  }

  /**
   * Calculates half of the centroid component for this edge. The twin edge's half should be
   * subtracted from this one to get the total centroid component.
   */
  private static float calculateHalfPartialCentroid(
      float sx, float sy, float scx, float scy, float ecx, float ecy, float ex, float ey) {
    return (scx * ecx * (45 * sy + 27 * scy)
        + scx * ex * (12 * sy + 18 * scy)
        + sx * scx * (105 * sy - 45 * scy - 45 * ecy - 15 * ey)
        + sx * ecx * (30 * sy)
        + sx * ex * (5 * sy + 3 * scy)
        + scx * scx * (45 * sy - 27 * ecy - 18 * ey)
        + sx * sx * (-280 * sy - 105 * scy - 30 * ecy - 5 * ey))
        / 840;
  }

  public Bubble getBubble() {
    return bubble;
  }

  public void setBubble(Bubble bubble) {
    this.bubble = bubble;
  }

  public Edge getNext() {
    return next;
  }

  public void setNext(Edge next) {
    this.next = next;
  }

  public Edge getPrev() {
    // This works because we know the graph is 3-regular.
    Edge prev = twin.next.twin.next.twin;
    if (prev.next != this) {
      throw new IllegalStateException("The graph is in an unstable state!");
    }
    return prev;
  }

  public Edge getTwin() {
    return twin;
  }

  public Vertex getStart() {
    return start;
  }

  private void setStart(Vertex start) {
    this.start = start;
    this.start.setEdge(this);
  }

  public Variable getStartCtrl() {
    return startCtrl;
  }

  public Variable getEndCtrl() {
    return twin.startCtrl;
  }

  public Vertex getEnd() {
    return twin.start;
  }

  public void setEnd(Vertex end) {
    twin.setStart(end);
  }

  public void makeFirstEdge() {
    bubble.setFirstEdge(this);
  }

  public Point getCenter() {
    float x = (2 * start.x + 2 * getEnd().x + startCtrl.x + getEndCtrl().x) / 6;
    float y = (2 * start.y + 2 * getEnd().y + startCtrl.y + getEndCtrl().y) / 6;
    return new Point(x, y);
  }

  public int getSimpleLength() {
    return (int) Math.hypot(start.x - getEnd().x, start.y - getEnd().y);
  }

  public double getPressure(double speedBump) {
    Bubble otherBubble = getTwin().getBubble();
    return getBubble().getPressureRatio(speedBump) - otherBubble.getPressureRatio(speedBump);
  }

  public double getArea() {
    return halfArea - twin.halfArea;
  }

  public double getCentroidComponentX() {
    return halfCentroidComponentX - twin.halfCentroidComponentX;
  }

  public double getCentroidComponentY() {
    return halfCentroidComponentY - twin.halfCentroidComponentY;
  }

  public void update() {
    halfCentroidComponentY = calculateHalfPartialCentroidY();
    halfCentroidComponentX = calculateHalfPartialCentroidX();
    halfArea = calculateHalfArea();
  }

  /**
   * Calculates half of the area. To get the total subtract the twin edge's half.
   */
  private float calculateHalfArea() {
    Point s = getStart();
    Point sc = getStartCtrl();
    Point ec = getEndCtrl();
    Point e = getEnd();

    return (s.x * (-10 * s.y - 6 * sc.y - 3 * ec.y - e.y) + sc.x * (6 * s.y - 3 * ec.y - 3 * e.y))
        / 20;
  }

  /** Calculates half of the partial Y Centroid. To get the total subtract the twin edge's half. */
  private double calculateHalfPartialCentroidY() {
    Point s = getStart();
    Point sc = getStartCtrl();
    Point ec = getEndCtrl();
    Point e = getEnd();

    return calculateHalfPartialCentroid(s.y, s.x, sc.y, sc.x, ec.y, ec.x, e.y, e.x);
  }

  /** Calculates half of the partial X Centroid. To get the total subtract the twin edge's half. */
  private double calculateHalfPartialCentroidX() {
    Point s = getStart();
    Point sc = getStartCtrl();
    Point ec = getEndCtrl();
    Point e = getEnd();

    return calculateHalfPartialCentroid(s.x, s.y, sc.x, sc.y, ec.x, ec.y, e.x, e.y);
  }

  // flattens this to a buffer *excluding the end point*
  public void flatten(FloatBuffer buffer) {
    Point s = getStart();
    Point sc = getStartCtrl();
    Point ec = getEndCtrl();
    Point e = getEnd();

    flatten(buffer, s.x, s.y, sc.x, sc.y, ec.x, ec.y, e.x, e.y);
  }

  private void flatten(
      FloatBuffer buffer,
      float x1,
      float y1,
      float x2,
      float y2,
      float x3,
      float y3,
      float x4,
      float y4) {
    if (!buffer.hasRemaining()) {
      return;
    }

    float dx = x4 - x1;
    float dy = y4 - y1;
    float d2 = Math.abs(((x2 - x4) * dy - (y2 - y4) * dx));
    float d3 = Math.abs(((x3 - x4) * dy - (y3 - y4) * dx));

    if ((d2 + d3) * (d2 + d3) < FLATNESS * (dx * dx + dy * dy)) {
      buffer.put(x1).put(y1);
      return;
    }

    // split in two by De Casteljau's Algorithm
    float x12 = (x1 + x2) / 2;
    float y12 = (y1 + y2) / 2;
    float x23 = (x2 + x3) / 2;
    float y23 = (y2 + y3) / 2;
    float x34 = (x3 + x4) / 2;
    float y34 = (y3 + y4) / 2;
    float x123 = (x12 + x23) / 2;
    float y123 = (y12 + y23) / 2;
    float x234 = (x23 + x34) / 2;
    float y234 = (y23 + y34) / 2;
    float x1234 = (x123 + x234) / 2;
    float y1234 = (y123 + y234) / 2;

    flatten(buffer, x1, y1, x12, y12, x123, y123, x1234, y1234);
    flatten(buffer, x1234, y1234, x234, y234, x34, y34, x4, y4);
  }
}
