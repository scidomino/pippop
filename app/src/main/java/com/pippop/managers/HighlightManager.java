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
      glowLine.update(bubble);
      g.drawLine(glowLine, Color.WHITE);
    }
  }

  private Bubble closestSwappable(Graph graph) {
    if (point == null) {
      return null;
    }

    Bubble playerBubble = graph.getPlayerBubble();
    Edge edge = closestEdge(playerBubble, point);
    if (edge == null) {
      return null;
    }
    Bubble bubble = edge.getTwin().getBubble();
    if (bubble instanceof OpenAir) {
      return null;
    }
    return bubble;
  }

  private Edge closestEdge(Bubble playerBubble, Point point) {
    for (Edge edge : playerBubble) {
      Bubble bubble = edge.getTwin().getBubble();
      if (bubble.contains(point) && !(bubble instanceof OpenAir)) {
        return edge;
      }
    }
    return null;
  }

  public void setPoint(Point point) {
    this.point = point;
  }
}
