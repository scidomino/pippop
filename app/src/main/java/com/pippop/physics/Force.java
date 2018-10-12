package com.pippop.physics;

import com.pippop.graph.Edge;

public interface Force {

  float getVertex(Edge edge);

  float getCtrl(Edge edge);
}
