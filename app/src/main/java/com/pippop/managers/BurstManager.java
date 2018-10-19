package com.pippop.managers;

import android.content.Context;
import android.media.MediaPlayer;
import com.pippop.R;
import com.pippop.graph.Bubble;
import com.pippop.graph.Edge;
import com.pippop.graph.Graph;
import com.pippop.graphics.Color;
import com.pippop.graphics.Graphics;
import com.pippop.graphics.Polyline;

public class BurstManager extends GraphManager {
  private static final int FREEZE_MILLISECONDS = 500;
  private static final int MAX_WIDTH = 20;
  private static final Color HIGHLIGHT_COLOR = Color.TRANSPARENT_WHITE;
  private final Polyline polyline = new Polyline(100);
  private final MediaPlayer sound;
  private Edge edge;
  private long timeLeft;

  public BurstManager(Context context) {
    this.sound = MediaPlayer.create(context, R.raw.burst);
  }

  public void update(Graph graph, int delta) {
    timeLeft -= delta;
  }

  public void render(Graphics g) {
    float percentDone = 1 - (timeLeft / (float) FREEZE_MILLISECONDS);
    for (int i = 0; i < MAX_WIDTH * percentDone; i++) {
      polyline.update(edge, i);
      g.drawLine(polyline, HIGHLIGHT_COLOR);
    }
  }

  public Edge burstEdge(Graph graph) {
    Edge removed = edge;

    burst(graph, removed);
    Bubble bubble = removed.getBubble();

    edge = null;

    Edge burstableEdge = findBurstableEdge(bubble);
    if (burstableEdge != null) {
      edge = burstableEdge;
      timeLeft = FREEZE_MILLISECONDS;
    }

    sound.seekTo(0);
    sound.start();

    return removed;
  }

  public boolean findAndSetBurstableEdges(Graph graph) {
    Edge burstStarter = findBurstStarter(graph);
    if (burstStarter == null) {
      return false;
    }
    edge = burstStarter;
    timeLeft = FREEZE_MILLISECONDS;
    return true;
  }

  public boolean isDone() {
    return timeLeft <= 0;
  }
}
