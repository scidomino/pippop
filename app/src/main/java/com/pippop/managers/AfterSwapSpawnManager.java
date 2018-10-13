package com.pippop.managers;

import com.pippop.graph.Graph;
import com.pippop.graphics.Color;
import com.pippop.util.RandomChooser;

public class AfterSwapSpawnManager extends SpawnManager {

  private final int minBubbles;
  private int bubblesToSpawn = 0;
  private int nextSpawnTime = 1000;

  public AfterSwapSpawnManager(RandomChooser<Color> colorChooser, int minBubbles) {
    super(colorChooser);
    this.minBubbles = minBubbles;
  }

  @Override
  public void update(Graph graph, int delta) {
    if (graph.getBubbles().size() < minBubbles) {
      nextSpawnTime -= delta;
    }
  }

  public void swapDone() {
    bubblesToSpawn += 2;
  }

  @Override
  public void possiblySpawn(Graph graph) {
    if (bubblesToSpawn > 0) {
      bubblesToSpawn--;
      spawn(graph, true);
    } else if (nextSpawnTime < 0) {
      spawn(graph, false);
      this.nextSpawnTime = 200;
    }
  }
}
