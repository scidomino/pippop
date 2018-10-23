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

  private static final int TEASER_DELAY = 4000;
  private static final int TEASER_THROB_TIME = 1000;
  private final GlowLine glowLine = new GlowLine(1000);
  private Point point;
  private int time;

  public void update(int delta) {
    this.time += delta;
  }

  public void render(Graph graph, Graphics g) {
    if (point != null) {
      Bubble bubble = closestSwappable(graph, point);
      if (bubble != null) {
        glowLine.update(bubble);
        g.drawLine(glowLine, Color.WHITE);
      }
    } else if (time % TEASER_DELAY > TEASER_DELAY / 2) {
      float ratio = (float) Math.pow(Math.sin(time * 2 * Math.PI / TEASER_THROB_TIME), 2);
      for (Edge edge : graph.getPlayerBubble()) {
        Bubble bubble = edge.getTwin().getBubble();
        if (!(bubble instanceof OpenAir)) {
          glowLine.update(bubble);
          g.drawLine(glowLine, Color.WHITE.withAlpha(ratio));
        }
      }
    }
  }

  private Bubble closestSwappable(Graph graph, Point point) {
    Edge edge = closestEdge(graph.getPlayerBubble(), point);
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
    this.time = 0;
  }
}
