package com.pippop.managers;

import com.pippop.graph.Bubble;
import com.pippop.graph.Edge;
import com.pippop.graph.Graph;
import com.pippop.graph.Point;
import com.pippop.graphics.Graphics;
import com.pippop.style.PlayerStyle;
import com.pippop.util.SwapPair;

public class SwapManager extends GraphManager {

  private SwapPair pair;

  public SwapManager() {}

  public boolean swap(Graph graph, Point point) {
    Bubble bubble = graph.getBubble(point);
    if (bubble == null) {
      return false;
    }
    Edge edge =
        bubble
            .stream()
            .filter(e -> e.getTwin().getBubble().getStyle() instanceof PlayerStyle)
            .findFirst()
            .orElse(null);

    if (edge == null) {
      return false;
    }

    //    Bubble top = edge.getTwin().getBubble();
    //    Bubble bot = edge.getBubble();
    //
    //    if (!(top.getStyle() instanceof GameStyle && bot.getStyle() instanceof GameStyle)) {
    //      return false;
    //    }

    this.pair = new SwapPair(edge, false);
    return true;
  }

  public void update(Graph graph, int delta) {
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
