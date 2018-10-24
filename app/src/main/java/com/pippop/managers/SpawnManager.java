package com.pippop.managers;

import android.content.Context;
import android.media.MediaPlayer;
import com.pippop.R;
import com.pippop.graph.Bubble;
import com.pippop.graph.Edge;
import com.pippop.graph.Graph;
import com.pippop.graph.Vertex;
import com.pippop.graphics.Color;
import com.pippop.style.GameStyle;
import com.pippop.util.RandomChooser;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Map.Entry;
import java.util.Random;
import java.util.Set;

public abstract class SpawnManager {

  private final RandomChooser<Color> colorChooser;
  private final Random random = new Random();
  private final MediaPlayer sound;

  SpawnManager(RandomChooser<Color> colorChooser, Context context) {
    this.colorChooser = colorChooser;
    this.sound = MediaPlayer.create(context, R.raw.spawn);
  }

  public abstract void update(Graph graph, int delta);

  public abstract void possiblySpawn(Graph graph);

  public void swapDone() {}

  public void reset(Graph graph) {
    graph.reset(new ArrayList<>(colorChooser.getAll()));
    sound.start();
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
    List<Vertex> verticies = new ArrayList<>();
    for (Edge edge : graph.getOpenAir()) {
      verticies.add(edge.getStart());
    }
    return verticies;
  }

  private <T> T getRandom(List<T> list) {
    return list.get(random.nextInt(list.size()));
  }

  private List<Vertex> findOpen(Map<Vertex, Set<Color>> map) {
    List<Vertex> vertices = new ArrayList<>();
    for (Entry<Vertex, Set<Color>> e : map.entrySet()) {
      if (e.getValue().size() < colorChooser.getSize()) {
        vertices.add(e.getKey());
      }
    }
    return vertices;
  }

  private void spawn(Graph graph, GameStyle gameStyle, Vertex vertex) {
    graph.spawn(vertex, gameStyle);
    sound.seekTo(0);
    sound.start();
  }

  private Map<Vertex, Set<Color>> createTouchingMap(Graph graph) {
    Map<Vertex, Set<Color>> map = new HashMap<>();
    for (Vertex vertex : graph.getVertices()) {
      map.put(vertex, new HashSet<Color>());
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
