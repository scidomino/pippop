package com.pippop.managers;

import com.pippop.graph.Edge;
import com.pippop.graph.Point;
import com.pippop.graphics.Color;
import com.pippop.graphics.Graphics;
import com.pippop.style.GameStyle;
import com.pippop.util.ChainTimer;
import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

public class ScoreManager {
  private static final float POINT_DISPLAY_TIME = 1000;
  private static final float POINT_MAX_HEIGHT = 150;
  private static final int WALL_BURST_POINTS = 10;
  private static final Color DISPLAY_COLOR = Color.WHITE;

  private final List<RisingPoints> risingPoints = new ArrayList<>();

  private final ChainTimer burstChainTimer = new ChainTimer(2000);
  private final ChainTimer popChainTimer = new ChainTimer(2000);

  private long score;

  public boolean isProcessing() {
    return !risingPoints.isEmpty();
  }

  public void update(int delta) {
    burstChainTimer.update(delta);
    popChainTimer.update(delta);

    Set<RisingPoints> landingPoints = new HashSet<>();
    for (RisingPoints flyingPoint : risingPoints) {
      if (flyingPoint.update(delta)) {
        landingPoints.add(flyingPoint);
      }
    }
    risingPoints.removeAll(landingPoints);
  }

  public long getScore() {
    return score;
  }

  public void render(Graphics g) {
    for (RisingPoints flyingPoint : risingPoints) {
      flyingPoint.render(g);
    }

    String value = "Score: " + String.valueOf(score);
    g.drawString(value, DISPLAY_COLOR, 150, 150);

    if (popChainTimer.getCount() > 1) {
      String chainString = popChainTimer.getCount() + " Pop Chain!";
      g.drawStringOutlined(chainString, DISPLAY_COLOR, Color.BLACK, 0, -150);
    } else if (burstChainTimer.getCount() > 1) {
      String chainString = burstChainTimer.getCount() + " Chain!";
      g.drawStringOutlined(chainString, DISPLAY_COLOR, Color.BLACK, 0, -150);
    }
  }

  public void onBurst(Edge edge) {
    burstChainTimer.reUp();

    int points = WALL_BURST_POINTS * burstChainTimer.getCount();

    addPoint(edge.getCenter(), points);
  }

  public void onPop(PoppedBubble popped) {
    popChainTimer.reUp();

    GameStyle gameStyle = popped.getStyle();
    int points = WALL_BURST_POINTS * gameStyle.getPoint();
    points *= popChainTimer.getCount();

    addPoint(popped.getCenter(), points);
  }

  private void addPoint(Point location, int points) {
    risingPoints.add(new RisingPoints(location, points));
    this.score += points;
  }

  private class RisingPoints {
    private final String text;
    private final int x;
    private final int y;

    private int time;

    RisingPoints(Point location, int points) {
      this.text = String.valueOf(points);
      this.x = (int) location.x;
      this.y = (int) location.y;
    }

    boolean update(int delta) {
      time += delta;
      return time > POINT_DISPLAY_TIME;
    }

    void render(Graphics g) {
      float rise = (time / POINT_DISPLAY_TIME) * POINT_MAX_HEIGHT;
      g.drawStringOutlined(this.text, DISPLAY_COLOR, Color.BLACK, (float) x, y + rise);
    }
  }
}
