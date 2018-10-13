package com.pippop.physics;

// import com.pippop.GameActivity;
import com.pippop.graph.Edge;
import com.pippop.graph.Graph;
import com.pippop.graph.Vertex;

class LookupTableBuilder {

  void reindex(Graph graph) {
    int index = 0;
    for (Vertex v : graph.getVertices()) {
      v.setIndex(index++);
    }
    index = 0;
    for (Edge e : graph.getEdges()) {
      e.getStartCtrl().setIndex(index++);
    }
  }

  void rebuildCtrlPoint2Vertex(int[] ctrlPoint2NearVertex, int[] ctrlPoint2FarVertex, Graph graph) {
    for (Edge edge : graph.getEdges()) {
      int i = edge.getStartCtrl().getIndex();

      ctrlPoint2NearVertex[i] = edge.getStart().getIndex();
      ctrlPoint2FarVertex[i] = edge.getEnd().getIndex();
    }
  }

  void rebuildVertex2CtrlPoint(
      int[][] vertex2NearCtrlPoints, int[][] vertex2FarCtrlPoints, Graph graph) {
    for (Vertex vertex : graph.getVertices()) {
      int i = vertex.getIndex();

      Edge edge = vertex.getEdge();
      Edge edge1 = edge.getTwin().getNext();
      Edge edge2 = edge1.getTwin().getNext();

      int[] near = vertex2NearCtrlPoints[i];
      near[0] = edge.getStartCtrl().getIndex();
      near[1] = edge1.getStartCtrl().getIndex();
      near[2] = edge2.getStartCtrl().getIndex();

      int[] far = vertex2FarCtrlPoints[i];
      far[0] = edge.getTwin().getStartCtrl().getIndex();
      far[1] = edge1.getTwin().getStartCtrl().getIndex();
      far[2] = edge2.getTwin().getStartCtrl().getIndex();
    }
  }

  void rebuildVertex2Vertex(int[][] vertex2Vertices, Graph graph) {
    for (Vertex vertex : graph.getVertices()) {
      int i = vertex.getIndex();
      if (i == 100) {
        System.exit(0);
      }

      Edge edge = vertex.getEdge();
      Edge edge1 = edge.getTwin().getNext();
      Edge edge2 = edge1.getTwin().getNext();

      vertex2Vertices[i][0] = edge.getEnd().getIndex();
      vertex2Vertices[i][1] = edge1.getEnd().getIndex();
      vertex2Vertices[i][2] = edge2.getEnd().getIndex();
    }
  }
}
