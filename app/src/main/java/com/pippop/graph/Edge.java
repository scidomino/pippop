package com.pippop.graph;

/**
 * Represents one side of a bubble wall. Together with it's twin it describes a cubic bezier curve
 * with control points (start, startCtrl, twin.startCtrl, twin.start) sometimes referred to as
 * (start, startCtrl, endCtrl, end).
 *
 * <p>User: Tommaso Sciortino Date: Oct 16, 2011 Time: 9:57:10 AM
 */
public class Edge {
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

  public void setStart(Vertex start) {
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

  /** Calculates half of the area. To get the total subtract the twin edges half. */
  public double calculateHalfArea() {
    double sx = getStart().x;
    double sy = getStart().y;
    double scx = getStartCtrl().x;
    double scy = getStartCtrl().y;

    double ecy = getEndCtrl().y;
    double ey = getEnd().y;

    double area = sx * (-10 * sy - 6 * scy - 3 * ecy - ey);
    area += scx * (6 * sy - 3 * ecy - 3 * ey);
    return area / 20;
  }

  public double calculateHalfPartialCentroidY() {
    double sx = getStart().x;
    double sy = getStart().y;
    double scx = getStartCtrl().x;
    double scy = getStartCtrl().y;

    double ecx = getEndCtrl().x;
    double ecy = getEndCtrl().y;
    double ex = getEnd().x;
    double ey = getEnd().y;

    return calculateHalfPartialCentroid(sy, sx, scy, scx, ecy, ecx, ey, ex);
  }

  public double calculateHalfPartialCentroidX() {
    double sx = getStart().x;
    double sy = getStart().y;
    double scx = getStartCtrl().x;
    double scy = getStartCtrl().y;

    double ecx = getEndCtrl().x;
    double ecy = getEndCtrl().y;
    double ex = getEnd().x;
    double ey = getEnd().y;

    return calculateHalfPartialCentroid(sx, sy, scx, scy, ecx, ecy, ex, ey);
  }

  /**
   * Calculates half of the centroid component for this edge. The twin edge's half should be
   * subtracted from this one to get the total centroid component.
   */
  private static double calculateHalfPartialCentroid(
      double sx, double sy, double scx, double scy, double ecx, double ecy, double ex, double ey) {
    double area = scx * ecx * (45 * sy + 27 * scy);
    area += scx * ex * (12 * sy + 18 * scy);
    area += sx * scx * (105 * sy - 45 * scy - 45 * ecy - 15 * ey);
    area += sx * ecx * (30 * sy);
    area += sx * ex * (5 * sy + 3 * scy);
    area += scx * scx * (45 * sy - 27 * ecy - 18 * ey);
    area += sx * sx * (-280 * sy - 105 * scy - 30 * ecy - 5 * ey);
    return area / 840;
  }
}
