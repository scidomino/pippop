package com.pippop.managers;

import com.pippop.graph.Edge;
import com.pippop.graph.Graph;
import java.util.HashMap;
import java.util.Iterator;
import java.util.Map;
import java.util.Map.Entry;

public class SlideManager extends GraphManager {

  private static final long TIMEOUT = 1000;
  private static final int MIN_LENGTH = 10;

  private final Map<Edge, Long> recentlySlid = new HashMap<Edge, Long>();

  public void slideSlidableEdges(Graph graph, int delta) {
    prune(delta);
    Edge edge = getFirstSlidable(graph);
    if (edge != null) {
      graph.slide(edge);
      this.recentlySlid.put(edge, TIMEOUT);
      burstAll(graph);
    }
  }

  private void prune(int delta) {
    Iterator<Entry<Edge, Long>> iterator = recentlySlid.entrySet().iterator();
    while (iterator.hasNext()) {
      Entry<Edge, Long> e = iterator.next();
      long val = e.getValue() - delta;
      if (val < 0) {
        iterator.remove();
      } else {
        e.setValue(val);
      }
    }
  }

  private Edge getFirstSlidable(Graph graph) {
    for (Edge edge : graph.getEdges()) {
      if (edge.getSimpleLength() < MIN_LENGTH
          && !this.recentlySlid.containsKey(edge)
          && !this.recentlySlid.containsKey(edge.getTwin())) {
        return edge;
      }
    }
    return null;
  }
}
