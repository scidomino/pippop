package com.pippop.managers;

import com.pippop.graph.Bubble;
import com.pippop.graph.Edge;
import com.pippop.graph.Graph;
import com.pippop.graph.Point;
import com.pippop.graphics.Graphics;
import com.pippop.util.SwapPair;

public class SwapManager {

  private SwapPair pair;

  public SwapManager() {}

  public boolean swap(Graph graph, Point point) {
    Bubble playerBubble = graph.getPlayerBubble();
    if (playerBubble.contains(point)) {
      return false;
    }

    Edge edge = graph.getClosestSwappable(point);
    if (edge != null) {
      this.pair = new SwapPair(edge, false);
      return true;
    }
    return false;
  }

  public void update(int delta) {
    this.pair.move(delta);
    if (this.pair.isDone()) {
      this.pair.switchBubbleProps();
      this.pair = null;
    }
  }

  public void render(Graphics g) {
    this.pair.draw(g);
  }

  public boolean isDone() {
    return this.pair == null;
  }
}
