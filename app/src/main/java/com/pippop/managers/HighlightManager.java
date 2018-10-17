package com.pippop.managers;

import com.pippop.graph.Bubble;
import com.pippop.graph.Edge;
import com.pippop.graph.Graph;
import com.pippop.graph.OpenAir;
import com.pippop.graph.Point;
import com.pippop.graphics.Color;
import com.pippop.graphics.Graphics;

public class HighlightManager {
  private Point point;
  private Edge edge;

  public void render(Graph graph, Graphics g) {
    processPoint(graph);

    if (this.edge != null) {
      Bubble top = this.edge.getTwin().getBubble();
      Bubble bottom = this.edge.getBubble();
      if (top == null || bottom == null) {
        this.edge = null;
      } else {
        drawHighlight(top, g);
        drawHighlight(bottom, g);
      }
    }
  }

  private void drawHighlight(Bubble bubble, Graphics g) {
    g.draw(bubble.getBuffer(), Color.RED, 7);
  }

  private void processPoint(Graph graph) {
    if (point != null) {
      Edge candidate = graph.getEdge(point);

      if (candidate == null
          || graph.getBubbles().size() <= 3
          || candidate.getBubble() instanceof OpenAir
          || candidate.getTwin().getBubble() instanceof OpenAir) {
        this.edge = null;
      } else {
        this.edge = candidate;
      }
      this.point = null;
    }
  }

  public void hover(Point point) {
    this.point = point;
  }

  public void killHighlight() {
    edge = null;
  }
}
