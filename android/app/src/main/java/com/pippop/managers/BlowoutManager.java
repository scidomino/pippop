package com.pippop.managers;

import com.pippop.graph.Graph;
import com.pippop.graphics.Color;

/** User: Tommaso Sciortino Date: Mar 30, 2011 Time: 8:46:00 PM */
public class BlowoutManager {
  private static final double MAX_AREA = 120000;
  private static final double PERIL_THRESHOLD = .8;

  private boolean isGameOver = false;
  private double flash;
  private Color outlineColor = Color.WHITE;

  public void reset() {
    this.isGameOver = false;
  }

  public void update(Graph graph, int delta) {
    double percentageFilled = getPercentageFilled(graph);
    if (percentageFilled > 1) {
      isGameOver = true;
      return;
    }
    if (percentageFilled < PERIL_THRESHOLD) {
      outlineColor = Color.WHITE;
    } else {
      flash += (delta + 200 * percentageFilled) / 300;
      double whiteness = 1 - percentageFilled * (1 + Math.sin(flash)) / 2;
      outlineColor = new Color(1f, (float) whiteness, (float) whiteness, 1f);
    }
  }

  public boolean isGameOver() {
    return isGameOver;
  }

  public Color getColor() {
    return outlineColor;
  }

  private double getPercentageFilled(Graph graph) {
    return -graph.getOpenAir().getArea() / MAX_AREA;
  }
}
