package com.pippop.managers;

import com.pippop.graph.Edge;
import com.pippop.graph.Graph;

public class PopSuccessManager implements SuccessManager {

  private final int popsRequired;
  private int pops;

  @Override
  public void reset() {
    pops = 0;
  }

  public PopSuccessManager(int popsRequired) {
    this.popsRequired = popsRequired;
  }

  @Override
  public boolean hasSucceeded(Graph graph) {
    return pops >= popsRequired;
  }

  @Override
  public void onBurst(Edge edge) {}

  @Override
  public void onPop(PoppedBubble bubble) {
    pops++;
  }
}
