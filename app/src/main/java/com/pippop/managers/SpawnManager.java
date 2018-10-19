package com.pippop.managers;

import static java.util.stream.Collectors.toList;
import static java.util.stream.Collectors.toMap;
import static java.util.stream.Collectors.toSet;

import android.content.Context;
import android.media.MediaPlayer;
import com.pippop.R;
import com.pippop.graph.Edge;
import com.pippop.graph.Graph;
import com.pippop.graph.Vertex;
import com.pippop.graphics.Color;
import com.pippop.style.GameStyle;
import com.pippop.style.PlayerStyle;
import com.pippop.util.RandomChooser;
import java.util.List;
import java.util.Map;
import java.util.Map.Entry;
import java.util.Random;
import java.util.Set;
import java.util.stream.Stream;

public abstract class SpawnManager extends GraphManager {

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
    Color color1 = colorChooser.chooseRanom();
    double angle = random.nextFloat() * Math.PI;

    PlayerStyle style1 = new PlayerStyle(1, Color.WHITE);
    GameStyle style2 = new GameStyle(1, color1);
    graph.reset(style1, style2, 0, 0, angle);
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
    return graph.getOpenAir().stream().map(Edge::getStart).collect(toList());
  }

  private <T> T getRandom(List<T> list) {
    return list.get(random.nextInt(list.size()));
  }

  private List<Vertex> findOpen(Map<Vertex, Set<Color>> map) {
    return map.entrySet()
        .stream()
        .filter(e -> e.getValue().size() < colorChooser.getSize())
        .map(Entry::getKey)
        .collect(toList());
  }

  private void spawn(Graph graph, GameStyle gameStyle, Vertex vertex) {
    graph.spawn(vertex, gameStyle);
    sound.start();
  }

  private Map<Vertex, Set<Color>> createTouchingMap(Graph graph) {
    return graph
        .getVertices()
        .stream()
        .collect(
            toMap(
                v -> v,
                v1 -> {
                  Edge e1 = v1.getEdge().getTwin();
                  Edge e2 = e1.getNext().getTwin();
                  Edge e3 = e2.getNext().getTwin();
                  return Stream.of(e1, e2, e3)
                      .map(e -> e.getBubble().getStyle())
                      .filter(s -> s instanceof GameStyle)
                      .map(s -> ((GameStyle) s).getColor())
                      .collect(toSet());
                }));
  }
}
