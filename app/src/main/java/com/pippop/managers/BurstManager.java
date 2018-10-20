package com.pippop.managers;

import static java.util.stream.Collectors.toSet;

import android.content.Context;
import android.media.MediaPlayer;
import com.pippop.R;
import com.pippop.graph.Bubble;
import com.pippop.graph.Edge;
import com.pippop.graph.Graph;
import com.pippop.graphics.Color;
import com.pippop.graphics.Graphics;
import com.pippop.graphics.Polyline;
import com.pippop.style.GameStyle;
import com.pippop.style.Style;
import java.util.Collection;

public class BurstManager {
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

  void burstAll(Graph graph) {
    Edge burstStarter = findBurstStarter(graph);
    while (burstStarter != null) {
      burst(graph, burstStarter);
      Bubble bubble = burstStarter.getBubble();

      Edge edge = findBurstableEdge(bubble);
      while (edge != null) {
        burst(graph, edge);
        edge = findBurstableEdge(bubble);
      }

      burstStarter = findBurstStarter(graph);
    }
  }

  void burst(Graph graph, Edge edge) {
    if (!isBurstable(edge)) {
      throw new IllegalStateException("edge is not burstable");
    }

    Bubble top = edge.getBubble();
    GameStyle topStyle = (GameStyle) top.getStyle();
    Bubble bottom = edge.getTwin().getBubble();
    GameStyle bottomStyle = (GameStyle) bottom.getStyle();

    top.setStyle(topStyle.combine(bottomStyle));

    graph.detach(edge);
  }

  Edge findBurstStarter(Graph graph) {
    if (graph.getBubbles().size() <= 5) {
      return null;
    }

    return graph
        .getBubbles()
        .stream()
        .map(b -> b.stream().filter(this::isBurstable).collect(toSet()))
        .filter(burstableEdges -> burstableEdges.size() >= 2)
        .flatMap(Collection::stream)
        .findAny()
        .orElse(null);
  }

  Edge findBurstableEdge(Bubble bubble) {
    return bubble.stream().filter(this::isBurstable).findAny().orElse(null);
  }

  private boolean isBurstable(Edge edge) {
    Bubble top = edge.getTwin().getBubble();
    Bubble bottom = edge.getBubble();
    if (top == bottom) {
      return false;
    }

    Style s1 = top.getStyle();
    Style s2 = bottom.getStyle();
    if (!(s1 instanceof GameStyle && s2 instanceof GameStyle)) {
      return false;
    }
    GameStyle style1 = (GameStyle) s1;
    GameStyle style2 = (GameStyle) s2;

    return style1.getColor().equals(style2.getColor());
  }
}
