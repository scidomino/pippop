package com.pippop.util;

public class GameStats {

  private int swaps;
  private int wallsBurst;
  private int bubblesPopped;
  private int maxBurstChain;
  private int maxPopChain;

  public void onSwap() {
    swaps++;
  }

  public void onWallBurst(int chainlevel) {
    wallsBurst++;
    maxBurstChain = Math.max(maxBurstChain, chainlevel);
  }

  public void onBubblePopped(int chainlevel) {
    bubblesPopped++;
    maxPopChain = Math.max(maxPopChain, chainlevel);
  }

  public int getSwaps() {
    return swaps;
  }

  public int getWallsBurst() {
    return wallsBurst;
  }

  public int getBubblesPopped() {
    return bubblesPopped;
  }

  public int getMaxBurstChain() {
    return maxBurstChain;
  }

  public int getMaxPopChain() {
    return maxPopChain;
  }
}
