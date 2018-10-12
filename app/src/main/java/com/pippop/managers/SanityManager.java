package com.pippop.managers;

import com.pippop.graph.Bubble;
import com.pippop.graph.Edge;
import com.pippop.graph.Graph;
import com.pippop.graph.OpenAir;
import com.pippop.graph.Vertex;

import java.util.HashSet;
import java.util.List;
import java.util.Set;

public class SanityManager {

  public void sanityCheck(Graph graph) {
    Set<Vertex> vertices = new HashSet<Vertex>();
    Set<Edge> edges = new HashSet<Edge>();
    if (!(graph.getBubbles().get(0) instanceof OpenAir)) {
      throw new RuntimeException("First bubble is not open air");
    }

    int openAirs = 0;
    for (Bubble bubble : graph.getBubbles()) {
      if (!graph.getEdges().contains(bubble.getFirstEdge())) {
        throw new RuntimeException("Bubble pointing to edge that's out of play!");
      }

      int totalEdges = 0;
      for (Edge edge : bubble) {
        totalEdges++;
        edges.add(edge);
        vertices.add(edge.getStart());

        if (edge.getEnd() != edge.getNext().getStart()) {
          throw new RuntimeException("First doesn't match last!");
        }

        if (!graph.getEdges().contains(edge)) {
          throw new RuntimeException("Undocumented Edge!");
        }

        if (edge.getBubble() != bubble) {
          throw new RuntimeException("Misforgetful Edge!");
        }
      }

      if (bubble instanceof OpenAir) {
        openAirs++;
      }

      if (totalEdges < 2) {
        throw new RuntimeException("All bubbles must have at least 2 edges");
      }
    }

    if (!edges.containsAll(graph.getEdges())) {
      throw new RuntimeException("Phantom edges!" + edges.size() + ":" + graph.getEdges().size());
    }

    List<Vertex> vertexList = graph.getVertices();
    if (!vertexList.containsAll(vertices)) {
      throw new RuntimeException("Vertex list doesn't contain all those in play");
    } else if (!vertices.containsAll(vertexList)) {
      throw new RuntimeException("Vertex list contain some that are not in play");
    }

    if (openAirs != 1) {
      throw new RuntimeException("Wrong number of open airs: " + openAirs);
    }

    int bubbleSize = graph.getBubbles().size();
    if ((bubbleSize - 2) * 2 != vertices.size()) {
      throw new RuntimeException(
          "vertices don't match bubbles: " + (bubbleSize - 2) * 2 + "!=" + vertices.size());
    }

    if (6 * (bubbleSize - 2) != edges.size()) {
      throw new RuntimeException(
          "edges don't match bubbles: " + 6 * (bubbleSize * -2) + "!=" + edges.size());
    }

    for (Edge edge : edges) {
      testEdge(edge);
    }
  }

  private void testEdge(Edge edge) {
    if (edge == null) {
      throw new RuntimeException("null edge");
    }
    if (edge.getBubble() == null) {
      throw new RuntimeException("null edge bubble");
    }
  }
}
