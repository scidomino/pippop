package com.pippop.managers;

import com.pippop.graph.Edge;
import com.pippop.graph.Graph;
import com.pippop.util.PoppedBubble;

public class ImpossibleSuccessManager implements SuccessManager {

  @Override
  public void reset() {}

  @Override
  public boolean hasSucceeded(Graph graph) {
    return false;
  }

  @Override
  public void onBurst(Edge edge) {}

  @Override
  public void onPop(PoppedBubble bubble) {}
}
