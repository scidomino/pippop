package com.pippop.managers;

import com.pippop.graph.Bubble;
import com.pippop.graph.Edge;
import com.pippop.graph.Graph;
import com.pippop.graph.Vertex;
import com.pippop.graphics.Color;
import com.pippop.style.GameStyle;
import com.pippop.style.PlayerStyle;
import com.pippop.util.RandomChooser;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Map.Entry;
import java.util.Random;
import java.util.Set;

public abstract class SpawnManager extends GraphManager {

  //	private final Sound spawn = ResourceManager.get().getSpawn();
  private final RandomChooser<Color> colorChooser;
  private final Random random = new Random();

    SpawnManager(RandomChooser<Color> colorChooser) {
    this.colorChooser = colorChooser;
  }

  public abstract void update(Graph graph, int delta);

  public abstract void possiblySpawn(Graph graph);

  public void swapDone() {}

  public void reset(Graph graph) {
    Color color1 = colorChooser.chooseRanom();
    double angle = random.nextFloat() * Math.PI;

      PlayerStyle style1 = new PlayerStyle(1, Color.WHITE);
      GameStyle style2 = new GameStyle(1, color1);
    graph.reset(style1, style2, 0, 0, angle);
    //		spawn.play();
  }

    void spawn(Graph graph, boolean openAirOnly) {
    Vertex vertex;
    Color color;

    Map<Vertex, Set<Color>> map = createTouchingMap(graph);
    List<Vertex> vertices = findOpen(map);

    if (openAirOnly) {
      vertices.retainAll(getOpenAirVertices(graph));
    }

    if (!vertices.isEmpty()) {
      vertex = getRandom(vertices);
      color = colorChooser.chooseRanom(map.get(vertex));
    } else {
      vertex = getRandom(graph.getVertices());
      if (openAirOnly) {
        vertices.retainAll(getOpenAirVertices(graph));
      }
      color = colorChooser.chooseRanom();
    }

    spawn(graph, new GameStyle(1, color), vertex);
  }

  private List<Vertex> getOpenAirVertices(Graph graph) {
      List<Vertex> list = new ArrayList<>();
    for (Edge edge : graph.getOpenAir()) {
      list.add(edge.getStart());
    }
    return list;
  }

  private <T> T getRandom(List<T> list) {
    return list.get(random.nextInt(list.size()));
  }

  private List<Vertex> findOpen(Map<Vertex, Set<Color>> map) {
      List<Vertex> list = new ArrayList<>();
    for (Entry<Vertex, Set<Color>> entry : map.entrySet()) {
      if (entry.getValue().size() < colorChooser.getSize()) {
        list.add(entry.getKey());
      }
    }
    return list;
  }

  private void spawn(Graph graph, GameStyle gameStyle, Vertex vertex) {
    graph.spawn(vertex, gameStyle);
    //		spawn.play();
  }

  private Map<Vertex, Set<Color>> createTouchingMap(Graph graph) {
      Map<Vertex, Set<Color>> map = new HashMap<>();
    for (Vertex vertex : graph.getVertices()) {
        map.put(vertex, new HashSet<>());
    }

    for (Edge edge : graph.getEdges()) {
      Set<Color> vertexColors = map.get(edge.getStart());
      addBubbleColor(vertexColors, edge.getBubble());
      addBubbleColor(vertexColors, edge.getNext().getTwin().getBubble());
    }
    return map;
  }

  private void addBubbleColor(Set<Color> set, Bubble bubble) {
    if (bubble.getStyle() instanceof GameStyle) {
      GameStyle gameStyle = (GameStyle) bubble.getStyle();
      set.add(gameStyle.getColor());
    }
  }
}
