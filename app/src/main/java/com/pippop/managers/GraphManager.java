package com.pippop.managers;

import com.pippop.graph.Bubble;
import com.pippop.graph.Edge;
import com.pippop.graph.Graph;
import com.pippop.style.GameStyle;
import com.pippop.style.Style;
import java.util.HashSet;
import java.util.Set;

class GraphManager {

  void burstAll(Graph graph) {
    Edge burstStarter = findBurstStarter(graph);
    while (burstStarter != null) {
      burst(graph, burstStarter);
      Bubble bubble = burstStarter.getBubble();

      Edge edge = findBurstableEdge(bubble);
      while (edge != null) {
        burst(graph, edge);
        edge = findBurstableEdge(bubble);
      }

      burstStarter = findBurstStarter(graph);
    }
  }

  void burst(Graph graph, Edge edge) {
    if (!isBurstable(edge)) {
      throw new IllegalStateException("edge is not burstable");
    }

    Bubble top = edge.getBubble();
    GameStyle topStyle = (GameStyle) top.getStyle();
    Bubble bottom = edge.getTwin().getBubble();
    GameStyle bottomStyle = (GameStyle) bottom.getStyle();

    top.setStyle(topStyle.combine(bottomStyle));

    graph.detach(edge);
  }

  Edge findBurstStarter(Graph graph) {
    if (graph.getBubbles().size() <= 5) {
      return null;
    }

    for (Bubble bubble : graph.getBubbles()) {
      Set<Bubble> matching = new HashSet<>();
      matching.add(bubble);
      Set<Edge> couldBurst = new HashSet<>();
      for (Edge edge : bubble) {
        if (!couldBurst.contains(edge.getTwin()) && isBurstable(edge)) {
          couldBurst.add(edge);
          matching.add(edge.getTwin().getBubble());
        }
      }
      if (matching.size() >= 3) {
        return couldBurst.iterator().next();
      }
    }

    return null;
  }

  Edge findBurstableEdge(Bubble bubble) {
    for (Edge edge : bubble) {
      if (isBurstable(edge)) {
        return edge;
      }
    }
    return null;
  }

  private boolean isBurstable(Edge edge) {
    Bubble top = edge.getTwin().getBubble();
    Bubble bottom = edge.getBubble();

    return canCombine(top.getStyle(), bottom.getStyle());
  }

  private boolean canCombine(Style s1, Style s2) {
    if (!(s1 instanceof GameStyle && s2 instanceof GameStyle)) {
      return false;
    }
    GameStyle style1 = (GameStyle) s1;
    GameStyle style2 = (GameStyle) s2;

    return style1.getColor().equals(style2.getColor());
  }
}
