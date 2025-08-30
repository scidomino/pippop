package com.pippop.graph;

public class Point {

  public float x;
  public float y;

  public Point(float x, float y) {
    this.x = x;
    this.y = y;
  }

  public void set(float x, float y) {
    this.x = x;
    this.y = y;
  }

  public void set(Point point) {
    set(point.x, point.y);
  }

  @Override
  public String toString() {
    return "Point [x=" + x + ", y=" + y + "]";
  }
}
