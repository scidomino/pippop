package com.pippop.physics;

import com.pippop.graph.Bubble;
import com.pippop.graph.Edge;
import com.pippop.graph.Graph;
import com.pippop.graph.Variable;
import com.pippop.graph.Vertex;

/** User: Tommaso Sciortino Date: Dec 5, 2011 Time: 6:22:02 PM */
public class PhysicsModel {

  private static final int MAX_VERTICES = 100;
  private static final float FRICTION = .9f;

  private static final float EIGHTY_THIRDS = 80f / 3f;
  private static final float FOUR_THIRDS = 4f / 3f;
  private static final float TWO_THIRDS = 2f / 3f;

  private final Graph graph;
  private final ForceCalculator forceCalculator = new ForceCalculator();
  private final LookupTableBuilder lookupTableBuilder = new LookupTableBuilder();

  private final int[][] vertex2NearCtrlPoints = new int[MAX_VERTICES][3];
  private final int[][] vertex2FarCtrlPoints = new int[MAX_VERTICES][3];
  private final int[] ctrlPoint2NearVertex = new int[3 * MAX_VERTICES];
  private final int[] ctrlPoint2FarVertex = new int[3 * MAX_VERTICES];
  private final int[][] vertex2Vertex = new int[MAX_VERTICES][3];

  private final float[] vertexForce = new float[MAX_VERTICES];
  private final float[] ctrlPointForce = new float[3 * MAX_VERTICES];

  private final float[] fbaf = new float[MAX_VERTICES];

  private final float[] vertexAccelX = new float[MAX_VERTICES];
  private final float[] vertexAccelY = new float[MAX_VERTICES];

  private final float[] ctrlPointAccelX = new float[3 * MAX_VERTICES];
  private final float[] ctrlPointAccelY = new float[3 * MAX_VERTICES];

  public PhysicsModel(Graph graph) {
    this.graph = graph;
  }

  public void update() {
    lookupTableBuilder.reindex(graph);
    lookupTableBuilder.rebuildVertex2Vertex(vertex2Vertex, graph);
    lookupTableBuilder.rebuildVertex2CtrlPoint(vertex2NearCtrlPoints, vertex2FarCtrlPoints, graph);
    lookupTableBuilder.rebuildCtrlPoint2Vertex(ctrlPoint2NearVertex, ctrlPoint2FarVertex, graph);
  }

  public void move(float centerX, float centerY) {
    forceCalculator.calculateForceX(vertexForce, ctrlPointForce, graph, centerX);
    solve(graph, vertexAccelX, ctrlPointAccelX);

    forceCalculator.calculateForceY(vertexForce, ctrlPointForce, graph, centerY);
    solve(graph, vertexAccelY, ctrlPointAccelY);

    acclerate(graph);
    updateShapes();
  }

  private void solve(Graph graph, float[] vertexAccel, float[] ctrlPointAccel) {
    simpleSolveVertices(graph, vertexAccel);
    solveCtrlPoints(graph, vertexAccel, ctrlPointAccel);
  }

  private void solveCtrlPoints(Graph graph, float[] vertexAccel, float[] ctrlPointAccel) {
    int size = graph.getEdges().size();
    for (int i = 0; i < size; i++) {
      int otherVertex = i % 2 == 0 ? i + 1 : i - 1;
      ctrlPointAccel[i] = EIGHTY_THIRDS * ctrlPointForce[i];
      ctrlPointAccel[i] -= 20 * ctrlPointForce[otherVertex];
      ctrlPointAccel[i] -= FOUR_THIRDS * vertexAccel[ctrlPoint2NearVertex[i]];
      ctrlPointAccel[i] += TWO_THIRDS * vertexAccel[ctrlPoint2FarVertex[i]];
    }
  }

  private void simpleSolveVertices(Graph graph, float[] vertexAccel) {
    for (Vertex v : graph.getVertices()) {
      int i = v.getIndex();
      fbaf[i] = vertexForce[i];
      for (int j : vertex2NearCtrlPoints[i]) {
        fbaf[i] -= FOUR_THIRDS * ctrlPointForce[j];
      }
      for (int j : vertex2FarCtrlPoints[i]) {
        fbaf[i] += TWO_THIRDS * ctrlPointForce[j];
      }
    }

    int size = graph.getVertices().size();
    for (int i = 0; i < size; i++) {
      int[] adjecent = vertex2Vertex[i];
      vertexAccel[i] =
          5.1f * fbaf[i] - .4f * (fbaf[adjecent[0]] + fbaf[adjecent[1]] + fbaf[adjecent[2]]);
    }
  }

  private void acclerate(Graph graph) {
    for (Vertex vertex : graph.getVertices()) {
      int index = vertex.getIndex();
      vertex.accelerate(vertexAccelX[index], vertexAccelY[index], FRICTION);
    }
    for (Edge edge : graph.getEdges()) {
      Variable ctrlPoint = edge.getStartCtrl();
      int index = ctrlPoint.getIndex();
      ctrlPoint.accelerate(ctrlPointAccelX[index], ctrlPointAccelY[index], FRICTION);
    }
  }

  private void updateShapes() {
    for (Edge edge : graph.getEdges()) {
      edge.update();
    }
    for (Bubble bubble : graph.getBubbles()) {
      bubble.update();
    }
  }
}
