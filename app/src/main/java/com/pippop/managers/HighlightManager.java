package com.pippop.managers;

import com.pippop.graph.Bubble;
import com.pippop.graph.Edge;
import com.pippop.graph.Graph;
import com.pippop.graph.OpenAir;
import com.pippop.graph.Point;
import com.pippop.graphics.Color;
import com.pippop.graphics.GlowLine;
import com.pippop.graphics.Graphics;
import com.pippop.style.PlayerStyle;
import java.util.List;

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
    Bubble playerBubble = findPlayerBubble(graph.getBubbles());
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

  private Bubble findPlayerBubble(List<Bubble> bubbles) {
    for (Bubble bubble : bubbles) {
      if (bubble.getStyle() instanceof PlayerStyle) {
        return bubble;
      }
    }
    throw new IllegalStateException("No player bubble!");
  }

  private Edge closestEdge(Bubble bubble, Point point) {
    double minDistance = 10000;
    Edge closest = null;
    for (Edge edge : bubble) {
      Point center = edge.getCenter();
      double distance = Math.hypot(point.x - center.x, point.y - center.y);
      if (distance < minDistance) {
        closest = edge;
        minDistance = distance;
      }
    }
    return closest;
  }

  public void setPoint(Point point) {
    this.point = point;
  }
}
