package com.pippop.managers;

import com.pippop.graph.Bubble;
import com.pippop.graph.Graph;
import com.pippop.graphics.Color;
import com.pippop.graphics.Graphics;

public class ShowAndMoveManager {

  public void update(Graph graph, float centerX, float centerY) {
    graph.getPhysicsModel().move(centerX, centerY);
  }

  public void render(Graphics graphics, Graph graph, Color outLineColor) {
    for (Bubble bubble : graph.getBubbles()) {
      bubble.render(graphics, outLineColor);
    }
  }
}
