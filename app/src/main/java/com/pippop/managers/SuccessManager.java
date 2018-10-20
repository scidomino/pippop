package com.pippop.managers;

import com.pippop.graph.Edge;
import com.pippop.graph.Graph;
import com.pippop.util.PoppedBubble;

public interface SuccessManager {

  void reset();

  boolean hasSucceeded(Graph graph);

  void onBurst(Edge edge);

  void onPop(PoppedBubble popped);
}
