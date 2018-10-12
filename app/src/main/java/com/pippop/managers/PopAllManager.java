package com.pippop.managers;

import com.pippop.graph.Bubble;
import com.pippop.graph.Edge;
import com.pippop.graph.Graph;
import com.pippop.graph.OpenAir;

import java.util.Set;

/** User: Tommaso Sciortino Date: Mar 30, 2011 Time: 8:46:00 PM */
public class PopAllManager {
  private static final int POP_DELAY = 100;

  private int timeTillNextPop;

  public void update(Graph graph, int delta) {
    if (graph.getBubbles().size() <= 3) {
      return;
    }
    if (timeTillNextPop > 0) {
      timeTillNextPop -= delta;
    } else {
      OpenAir air = graph.getOpenAir();
      Set<Bubble> touchingAir = air.getAdjacentBubbles();
      for (Edge edge : air) {
        Bubble bubble = edge.getTwin().getBubble();
        if (touchingAir.contains(bubble)) {
          graph.detach(edge);
          break;
        }
      }
      timeTillNextPop = POP_DELAY;
    }
  }
}
