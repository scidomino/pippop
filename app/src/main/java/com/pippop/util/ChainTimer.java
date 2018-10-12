package com.pippop.util;

public class ChainTimer {

  private final int resetTime;

  private int time;
  private int count = 0;

  public ChainTimer(int resetTime) {
    this.resetTime = resetTime;
  }

  public void reUp() {
    if (time > 0) {
      count++;
    } else {
      count = 1;
    }
    time = resetTime;
  }

  public void update(int delta) {
    time -= delta;
  }

  public int getCount() {
    if (time < 0) {
      return 0;
    }
    return count;
  }

  public float percentToExpiration() {
    if (time < 0) {
      return 0;
    } else {
      return time / (float) resetTime;
    }
  }
}
