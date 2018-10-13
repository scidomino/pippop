package com.pippop.physics;

import com.pippop.graph.Edge;

interface Force {

  float getVertex(Edge edge);

  float getCtrl(Edge edge);
}
