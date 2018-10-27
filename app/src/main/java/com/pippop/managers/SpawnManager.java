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
import com.pippop.style.PlayerStyle;
import com.pippop.util.RandomChooser;
import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
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

  public void reset(Graph graph, PlayerStyle centerStyle) {
    graph.reset(new ArrayList<>(colorChooser.getAll()), centerStyle);
    sound.start();
  }

  private void spawn(Graph graph, GameStyle gameStyle, Vertex vertex) {
    graph.spawn(vertex, gameStyle);
    sound.seekTo(0);
    sound.start();
  }

  void spawn(Graph graph) {
    Vertex vertex = getRandom(getOpenAirVertices(graph));

    Set<Color> colors = new HashSet<>();
    addColor(colors, vertex.getEdge().getBubble());
    addColor(colors, vertex.getEdge().getTwin().getBubble());
    addColor(colors, vertex.getEdge().getTwin().getNext().getTwin().getBubble());

    Color color = colorChooser.chooseRanom(colors);

    spawn(graph, new GameStyle(1, color), vertex);
  }

  private void addColor(Set<Color> colors, Bubble bubble) {
    if (bubble.getStyle() instanceof GameStyle) {
      colors.add(((GameStyle) bubble.getStyle()).getColor());
    }
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
}
