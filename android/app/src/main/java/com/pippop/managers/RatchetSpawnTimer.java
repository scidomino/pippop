package com.pippop.managers;

import com.pippop.managers.SpawnManager.SpawnTimer;

public class RatchetSpawnTimer implements SpawnTimer {

  private final int startingWait;
  private final int doublingTime;

  public RatchetSpawnTimer(int startingWait, int doublingTime) {
    this.startingWait = startingWait;
    this.doublingTime = doublingTime;
  }

  @Override
  public double getAverageWait(int bubbleCount, long totalPlayTime) {
    return startingWait * Math.pow(.5, totalPlayTime / (float) doublingTime);
  }
}
