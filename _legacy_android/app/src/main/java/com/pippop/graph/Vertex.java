package com.pippop.graph;

/** User: Tommaso Sciortino Date: Oct 16, 2011 Time: 9:37:10 AM */
public class Vertex extends Variable {

  // One of three edges for which this is the start vertex
  private Edge edge;

  public Vertex(float x, float y) {
    super(x, y);
  }

  public Edge getEdge() {
    return edge;
  }

  public void setEdge(Edge edge) {
    this.edge = edge;
  }
}
