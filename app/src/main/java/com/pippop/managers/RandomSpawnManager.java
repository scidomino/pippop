package com.pippop.managers;

import android.content.Context;
import com.pippop.graph.Graph;
import com.pippop.graphics.Color;
import com.pippop.util.RandomChooser;
import java.util.Random;

public class RandomSpawnManager extends SpawnManager {

  private final int minBubbles;
  private final Random random = new Random();
  private int nextSpawnTime = 1000;
  private long totalPlayTime = 0;

  public RandomSpawnManager(RandomChooser<Color> colorChooser, int minBubbles, Context context) {
    super(colorChooser, context);
    this.minBubbles = minBubbles;
  }

  @Override
  public void reset(Graph graph) {
    super.reset(graph);
    totalPlayTime = 0;
  }

  @Override
  public void update(Graph graph, int delta) {
    totalPlayTime += delta;
    nextSpawnTime -= delta;
  }

  @Override
  public void possiblySpawn(Graph graph) {
    if (nextSpawnTime < 0) {
      spawn(graph, false);
      this.nextSpawnTime = getNextSpawnTime(graph);
    }
  }

  private int getNextSpawnTime(Graph graph) {
    if (graph.getBubbles().size() < minBubbles) {
      return 1000;
    }
    double averageWait = 1000 * (1 + 1.5 * Math.exp(-totalPlayTime / 200000f));
    return (int) (-averageWait * Math.log(1 - random.nextDouble()));
  }
}
