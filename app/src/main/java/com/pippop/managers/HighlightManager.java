package com.pippop.managers;

import com.pippop.graph.Bubble;
import com.pippop.graph.Edge;
import com.pippop.graph.Graph;
import com.pippop.graph.OpenAir;
import com.pippop.graph.Point;
import com.pippop.graphics.Color;
import com.pippop.graphics.GlowLine;
import com.pippop.graphics.Graphics;

public class HighlightManager {

  private final GlowLine glowLine = new GlowLine(1000);
  private Point point;

  public void render(Graph graph, Graphics g) {
    Bubble bubble = closestSwappable(graph);
    if (bubble != null) {
      glowLine.update(bubble, 20);
      g.drawLine(glowLine, Color.RED);
    }
  }

  private Bubble closestSwappable(Graph graph) {
    if (point == null) {
      return null;
    }
    Bubble playerBubble = graph.getBubbles().get(1);

    Edge edge = playerBubble.getCorrespondingEdge(point);
    if (edge == null) {
      return null;
    }
    Bubble bubble = edge.getTwin().getBubble();
    if (bubble instanceof OpenAir) {
      return null;
    }
    return bubble;
  }

  public void setPoint(Point point) {
    this.point = point;
  }
}
