package com.pippop.graph;

import com.pippop.physics.PhysicsModel;

public class GraphChangeDetector {
  private PhysicsModel physicsModel;

  public boolean hasChanged(Graph graph) {
    if (graph.getPhysicsModel() == physicsModel) {
      return false;
    } else {
      physicsModel = graph.getPhysicsModel();
      return true;
    }
  }
}
