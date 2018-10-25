package com.pippop.graph;

import com.pippop.graphics.Color;
import com.pippop.graphics.Graphics;
import com.pippop.physics.PhysicsModel;
import com.pippop.style.GameStyle;
import com.pippop.style.PlayerStyle;
import com.pippop.style.Style;
import java.util.ArrayList;
import java.util.List;

/** User: Tommaso Sciortino Date: Oct 16, 2011 Time: 9:39:01 AM */
public class Graph {

  private final List<Vertex> vertices = new ArrayList<>();
  private final List<Edge> edges = new ArrayList<>();
  private final List<Bubble> bubbles = new ArrayList<>();

  private final PhysicsModel physicsModel = new PhysicsModel(this);
  private boolean isModelDirty = true;

  private static Edge split(Edge edge) {
    float x1 = edge.getStart().x;
    float y1 = edge.getStart().y;
    float ctrlx1 = edge.getStartCtrl().x;
    float ctrly1 = edge.getStartCtrl().y;
    float ctrlx2 = edge.getEndCtrl().x;
    float ctrly2 = edge.getEndCtrl().y;
    float x2 = edge.getEnd().x;
    float y2 = edge.getEnd().y;
    float centerx = (ctrlx1 + ctrlx2) / 2f;
    float centery = (ctrly1 + ctrly2) / 2f;
    ctrlx1 = (x1 + ctrlx1) / 2f;
    ctrly1 = (y1 + ctrly1) / 2f;
    ctrlx2 = (x2 + ctrlx2) / 2f;
    ctrly2 = (y2 + ctrly2) / 2f;
    float ctrlx12 = (ctrlx1 + centerx) / 2f;
    float ctrly12 = (ctrly1 + centery) / 2f;
    float ctrlx21 = (ctrlx2 + centerx) / 2f;
    float ctrly21 = (ctrly2 + centery) / 2f;
    centerx = (ctrlx12 + ctrlx21) / 2f;
    centery = (ctrly12 + ctrly21) / 2f;

    Vertex newVertex = new Vertex(centerx, centery);
    Edge newEdge = new Edge(newVertex, edge.getEnd());

    edge.setEnd(newVertex);

    edge.getStartCtrl().set(ctrlx1, ctrly1);
    edge.getEndCtrl().set(ctrlx12, ctrly12);

    newEdge.getStartCtrl().set(ctrlx21, ctrly21);
    newEdge.getEndCtrl().set(ctrlx2, ctrly2);

    newEdge.setNext(edge.getNext());
    edge.setNext(newEdge);

    newEdge.setBubble(edge.getBubble());

    edge.update();
    edge.getTwin().update();
    newEdge.update();
    newEdge.getTwin().update();

    return newEdge;
  }

  /**
   * This method has purposefully been given a confusing name to dissuade you from attempting to
   * comprehend it.
   */
  private static void knitAndPurl(Edge edge) {
    Edge purl = edge.getNext().getTwin().getNext();
    Edge ops = purl.getTwin().getPrev();

    edge.setNext(purl.getNext());
    ops.setNext(edge.getTwin());

    edge.getTwin().makeFirstEdge();
    edge.makeFirstEdge();

    edge.setEnd(purl.getEnd());
    edge.getStartCtrl().set(avg(edge.getStartCtrl(), purl.getStartCtrl()));
    edge.getEndCtrl().set(avg(edge.getEndCtrl(), purl.getEndCtrl()));
    edge.update();
    edge.getTwin().update();
  }

  private static Point avg(Variable o1, Variable o2) {
    float x = (o1.x + o2.x) / 2;
    float y = (o1.y + o2.y) / 2;
    return new Point(x, y);
  }

  public Edge getClosestSwappable(Point point) {
    Bubble playerBubble = getPlayerBubble();
    float minDistance = Float.MAX_VALUE;
    Edge closestEdge = null;
    for (Edge edge : playerBubble) {
      Bubble bubble = edge.getTwin().getBubble();
      if (!(bubble.getStyle() instanceof GameStyle)) {
        continue;
      }
      float distance = distance(bubble.getCenter(), point);
      if (distance < minDistance) {
        closestEdge = edge.getTwin();
        minDistance = distance;
      }
    }
    return closestEdge;
  }

  private float distance(Point a, Point b) {
    return (float) Math.hypot(a.x - b.x, a.y - b.y);
  }

  public List<Vertex> getVertices() {
    return vertices;
  }

  public List<Edge> getEdges() {
    return edges;
  }

  public List<Bubble> getBubbles() {
    return bubbles;
  }

  public PhysicsModel getPhysicsModel() {
    if (isModelDirty) {
      this.physicsModel.update();
      isModelDirty = false;
    }
    return physicsModel;
  }

  public OpenAir getOpenAir() {
    return (OpenAir) this.bubbles.get(0);
  }

  public Bubble getPlayerBubble() {
    for (Bubble bubble : bubbles) {
      if (bubble.getStyle() instanceof PlayerStyle) {
        return bubble;
      }
    }
    return null;
  }

  public void reset(Style style1, Style style2, int x, int y, double angle) {
    bubbles.clear();
    edges.clear();
    vertices.clear();

    int dist = 50;
    Vertex v1 = new Vertex((int) (x + dist * Math.cos(angle)), (int) (y + dist * Math.sin(angle)));
    Vertex v2 = new Vertex((int) (x - dist * Math.cos(angle)), (int) (y - dist * Math.sin(angle)));
    vertices.add(v1);
    vertices.add(v2);

    Edge edge1 = new Edge(v1, v2);
    Edge middleEdge = new Edge(v1, v2);
    Edge edge3 = new Edge(v1, v2);

    edge1.getTwin().setNext(edge3);
    edge3.setNext(edge1.getTwin());

    edge1.setNext(middleEdge.getTwin());
    middleEdge.getTwin().setNext(edge1);

    middleEdge.setNext(edge3.getTwin());
    edge3.getTwin().setNext(middleEdge);

    addBothSides(edge1);
    addBothSides(middleEdge);
    addBothSides(edge3);

    bubbles.add(new OpenAir(edge1.getTwin()));
    bubbles.add(new Bubble(style1, edge1));
    bubbles.add(new Bubble(style2, middleEdge));

    isModelDirty = true;
  }

  public void reset(List<Color> colors) {
    bubbles.clear();
    edges.clear();
    vertices.clear();

    int distance = 20;

    List<Edge> openAirEdges = new ArrayList<>();
    List<Edge> playerEdges = new ArrayList<>();

    Edge firstMiddleEdge = new Edge(new Vertex(2 * distance, 0), new Vertex(distance, 0));
    addBothSides(firstMiddleEdge);
    vertices.add(firstMiddleEdge.getStart());
    vertices.add(firstMiddleEdge.getEnd());

    Edge prevEdge = firstMiddleEdge.getTwin();
    for (int i = 1; i < colors.size(); i++) {
      double ratio = -(i * 2 * Math.PI / colors.size());

      float x = (float) (distance * Math.cos(ratio));
      float y = (float) (distance * Math.sin(ratio));

      Edge middleEdge = new Edge(new Vertex(2 * x, 2 * y), new Vertex(x, y));
      Edge outerEdge = new Edge(prevEdge.getEnd(), middleEdge.getStart());
      Edge innerEdge = new Edge(middleEdge.getEnd(), prevEdge.getStart());
      addBothSides(outerEdge);
      addBothSides(middleEdge);
      addBothSides(innerEdge);
      vertices.add(middleEdge.getStart());
      vertices.add(middleEdge.getEnd());

      prevEdge.setNext(outerEdge);
      outerEdge.setNext(middleEdge);
      middleEdge.setNext(innerEdge);
      innerEdge.setNext(prevEdge);

      bubbles.add(new Bubble(new GameStyle(1, colors.get(i)), innerEdge));

      openAirEdges.add(0, outerEdge.getTwin());
      playerEdges.add(innerEdge.getTwin());
      prevEdge = middleEdge.getTwin();
    }

    Edge outerEdge = new Edge(prevEdge.getEnd(), firstMiddleEdge.getStart());
    Edge innerEdge = new Edge(firstMiddleEdge.getEnd(), prevEdge.getStart());
    addBothSides(outerEdge);
    addBothSides(innerEdge);

    prevEdge.setNext(outerEdge);
    outerEdge.setNext(firstMiddleEdge);
    firstMiddleEdge.setNext(innerEdge);
    innerEdge.setNext(prevEdge);

    bubbles.add(new Bubble(new GameStyle(1, colors.get(0)), innerEdge));

    openAirEdges.add(outerEdge.getTwin());
    playerEdges.add(innerEdge.getTwin());

    for (int i = 0; i < playerEdges.size() - 1; i++) {
      playerEdges.get(i).setNext(playerEdges.get(i + 1));
      openAirEdges.get(i).setNext(openAirEdges.get(i + 1));
    }
    playerEdges.get(playerEdges.size() - 1).setNext(playerEdges.get(0));
    openAirEdges.get(openAirEdges.size() - 1).setNext(openAirEdges.get(0));

    bubbles.add(new Bubble(new PlayerStyle(1, Color.TRANSPARENT_WHITE), playerEdges.get(0)));
    bubbles.add(0, new OpenAir(openAirEdges.get(0)));

    isModelDirty = true;
  }

  public void spawn(Vertex vertex, Style style) {
    Edge e1 = vertex.getEdge().getTwin();
    Edge e2 = e1.getNext().getTwin();
    Edge e3 = e2.getNext().getTwin();

    Edge ne1 = split(e1);
    Edge ne2 = split(e2);
    Edge ne3 = split(e3);

    ne1.setEnd(e2.getEnd());
    ne2.setEnd(e3.getEnd());
    ne3.setEnd(e1.getEnd());

    ne1.setNext(e2.getTwin());
    ne2.setNext(e3.getTwin());
    ne3.setNext(e1.getTwin());

    ne1.getTwin().setNext(ne3.getTwin());
    ne2.getTwin().setNext(ne1.getTwin());
    ne3.getTwin().setNext(ne2.getTwin());

    ne1.update();
    ne1.getTwin().update();
    ne2.update();
    ne2.getTwin().update();
    ne3.update();
    ne3.getTwin().update();

    bubbles.add(new Bubble(style, ne1.getTwin(), Graphics.createVertexBuffer(1000)));

    addBothSides(ne1);
    addBothSides(ne2);
    addBothSides(ne3);

    vertices.add(ne1.getStart());
    vertices.add(ne2.getStart());
    vertices.add(ne3.getStart());

    vertices.remove(vertex);

    isModelDirty = true;
  }

  private void addBothSides(Edge edge) {
    edges.add(edge);
    edges.add(edge.getTwin());
  }

  public void detach(Edge edge) {
    Edge next = edge.getNext();
    Edge prev = edge.getPrev();

    Edge twin = edge.getTwin();
    Edge twinNext = twin.getNext();
    Edge twinPrev = twin.getPrev();

    Bubble absorber = edge.getBubble();
    Bubble absorbed = twin.getBubble();

    if (next == prev) {
      knitAndPurl(prev);
      knitAndPurl(twinPrev);
      edge.getBubble().setFirstEdge(twinPrev);
    } else if (twinNext == twinPrev) {
      knitAndPurl(prev);
      knitAndPurl(prev);
      edge.getBubble().setFirstEdge(prev);
    } else {
      knitAndPurl(prev);
      knitAndPurl(twinPrev);
      edge.getBubble().setFirstEdge(prev);
    }

    for (Edge absorbedEdge : absorbed) {
      absorbedEdge.setBubble(absorber);
    }

    absorber.update();

    vertices.remove(edge.getStart());
    vertices.remove(edge.getEnd());

    removeBothSides(edge);
    removeBothSides(next);
    removeBothSides(twinNext);

    bubbles.remove(absorbed);

    for (Edge degenerate : absorber.getDegenerateEdges()) {
      slide(degenerate);
    }

    isModelDirty = true;
  }

  private void removeBothSides(Edge edge) {
    edges.remove(edge);
    edges.remove(edge.getTwin());
  }

  public void slide(Edge edge) {
    // We have to retrieve all these ahead of time because once we start
    // monkeying with the graph it'll be impossible to determine correctly.
    Edge next = edge.getNext();
    Edge prev = edge.getPrev();

    Edge twin = edge.getTwin();
    Edge twinNext = twin.getNext();
    Edge twinPrev = twin.getPrev();

    if (prev.getTwin().getBubble() == twinPrev.getTwin().getBubble()) {
      // Though improbable, this will happen from time to time
      return;
    }

    halfSlide(edge, prev, twinNext);
    halfSlide(twin, twinPrev, next);

    isModelDirty = true;
  }

  private void halfSlide(Edge edge, Edge prev, Edge twinNext) {
    prev.setEnd(edge.getEnd());

    prev.setNext(edge.getNext());
    edge.setNext(prev.getTwin());
    twinNext.getTwin().setNext(edge);

    edge.setBubble(prev.getTwin().getBubble());

    prev.makeFirstEdge();
  }
}
