package com.pippop.managers;

import android.content.Context;
import android.media.MediaPlayer;
import com.pippop.R;
import com.pippop.graph.Bubble;
import com.pippop.graph.Edge;
import com.pippop.graph.Graph;
import com.pippop.graphics.Color;
import com.pippop.graphics.GlowLine;
import com.pippop.graphics.Graphics;
import com.pippop.style.GameStyle;
import com.pippop.style.Style;
import java.util.HashSet;
import java.util.Set;

public class BurstManager {
  private static final int FREEZE_MILLISECONDS = 500;
  private static final int MAX_WIDTH = 20;
  private static final Color HIGHLIGHT_COLOR = Color.TRANSPARENT_WHITE;
  private final GlowLine glowLine = new GlowLine();
  private final MediaPlayer sound;
  private final int threashhold;
  private Edge edge;
  private long timeLeft;

  public BurstManager(Context context, int threashhold) {
    this.sound = MediaPlayer.create(context, R.raw.burst);
    this.threashhold = threashhold;
  }

  public void update(int delta) {
    timeLeft -= delta;
  }

  public void render(Graphics g) {
    float percentDone = 1 - (timeLeft / (float) FREEZE_MILLISECONDS);
    glowLine.update(edge, MAX_WIDTH * percentDone);
    g.drawLine(glowLine, HIGHLIGHT_COLOR);
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

  private void burst(Graph graph, Edge edge) {
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

  private Edge findBurstStarter(Graph graph) {
    if (graph.getBubbles().size() <= 5) {
      return null;
    }
    for (Bubble bubble : graph.getBubbles()) {
      Set<Edge> burstable = findAllBurstable(bubble);
      if (burstable.size() >= threashhold) {
        return burstable.iterator().next();
      }
    }
    return null;
  }

  private Set<Edge> findAllBurstable(Bubble bubble) {
    Set<Edge> burstable = new HashSet<>();
    for (Edge edge : bubble) {
      if (isBurstable(edge)) {
        burstable.add(edge);
      }
    }
    return burstable;
  }

  private Edge findBurstableEdge(Bubble bubble) {
    for (Edge edge : bubble) {
      if (isBurstable(edge)) {
        return edge;
      }
    }
    return null;
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
