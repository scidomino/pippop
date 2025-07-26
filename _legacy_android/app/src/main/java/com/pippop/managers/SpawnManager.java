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
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashSet;
import java.util.List;
import java.util.Random;
import java.util.Set;

public class SpawnManager {

  private final Random random = new Random();
  private final MediaPlayer sound;
  private final SpawnTimer spawnTimer;
  private final List<Color> colors;

  private int nextSpawnTime;
  private long totalPlayTime = 0;

  public SpawnManager(int colorCount, SpawnTimer spawnTimer, Context context) {
    this.colors = Color.getGroup(colorCount);
    this.sound = MediaPlayer.create(context, R.raw.spawn);
    this.spawnTimer = spawnTimer;
    this.nextSpawnTime = getNextSpawnTime(3);
  }

  public void reset(Graph graph, PlayerStyle centerStyle) {
    graph.reset(colors, centerStyle);
    sound.seekTo(0);
    sound.start();
  }

  private void spawn(Graph graph, GameStyle gameStyle, Vertex vertex) {
    graph.spawn(vertex, gameStyle);
    sound.seekTo(0);
    sound.start();
  }

  private void spawn(Graph graph) {
    Vertex vertex = getRandom(getOpenAirVertices(graph));

    Set<Color> choices = getDistantColors(vertex);
    Color color = getRandom(new ArrayList<>(choices));
    spawn(graph, new GameStyle(1, color), vertex);
  }

  // Colors from bubbles touching this vertex or bubbles touching bubbles that touch this vertex.
  private Set<Color> getDistantColors(Vertex vertex) {
    Set<Color> colors = new HashSet<>(this.colors);

    Edge edge = vertex.getEdge();
    for (Edge e :
        Arrays.asList(
            edge,
            edge.getTwin(),
            edge.getPrev().getTwin(),
            edge.getNext().getTwin(),
            edge.getPrev().getPrev().getTwin(),
            edge.getTwin().getNext().getNext().getTwin())) {
      Bubble bubble = e.getBubble();
      if (bubble.getStyle() instanceof GameStyle) {
        colors.remove(((GameStyle) bubble.getStyle()).getColor());
      }
    }
    return colors;
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

  public void update(int delta) {
    totalPlayTime += delta;
    nextSpawnTime -= delta;
  }

  public void possiblySpawn(Graph graph) {
    if (nextSpawnTime < 0) {
      spawn(graph);
      this.nextSpawnTime = getNextSpawnTime(graph.getBubbles().size());
    }
  }

  private int getNextSpawnTime(int bubbleCount) {
    double averageWait = spawnTimer.getAverageWait(bubbleCount, totalPlayTime);
    return (int) (averageWait * -Math.log(random.nextDouble()));
  }

  public interface SpawnTimer {
    double getAverageWait(int bubbleCount, long totalPlayTime);
  }
}
