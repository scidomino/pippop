package com.pippop.graph;

public class Variable extends Point {

  private int index;

  private double xVel;
  private double yVel;

  public Variable(float x, float y) {
    super(x, y);
  }

  public int getIndex() {
    return index;
  }

  public void setIndex(int index) {
    this.index = index;
  }

  public void accelerate(double xAcceleration, double yAcceleration, double friction) {
    this.xVel = friction * this.xVel + xAcceleration;
    this.x += this.xVel;
    this.yVel = friction * this.yVel + yAcceleration;
    this.y += this.yVel;
  }
}
