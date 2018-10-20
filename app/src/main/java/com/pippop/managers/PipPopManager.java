package com.pippop.managers;

import com.pippop.graph.Bubble;
import com.pippop.graph.Graph;
import com.pippop.graphics.Graphics;
import com.pippop.style.GameStyle;

public class PipPopManager {

  private static final long TOTAL_TIME = 2000;

  private long timeLeft;

  public boolean hasPipPop(Graph graph) {
    int total = 0;
    for (Bubble bubble : graph.getBubbles()) {
      if (bubble.getStyle() instanceof GameStyle) {
        GameStyle style = (GameStyle) bubble.getStyle();
        if (style.isPoppable()) {
          total++;
        }
      }
    }
    return total >= 2;
  }

  public void reset() {
    timeLeft = TOTAL_TIME;
  }

  public void update(long delta) {
    timeLeft -= delta;
  }

  public boolean isDone() {
    return timeLeft < 0;
  }

  public void render(Graphics g) {
    //		float ratio = 1 - (timeLeft / (float) TOTAL_TIME);
    //		String pip = "Pip";
    //		String pop = "Pop";
    //		int pipWidth = g.getFont().getWidth(pip);
    //		int popWidth = g.getFont().getWidth(pop);
    //		int pipHeight = g.getFont().getHeight(pip);
    //
    //		if (ratio < .25) {
    //			float subRatio = ratio * 4;
    //			float pipX = (1 - subRatio) * -pipWidth + subRatio * (250 - pipWidth / 2);
    //			float popX = (1 - subRatio) * 500 + subRatio * (250 - popWidth / 2);
    //
    //			g.setColor(Color.WHITE);
    //			g.drawString(pip, pipX, 250 - pipHeight);
    //			g.drawString(pop, popX, 250);
    //		} else {
    //			float subRatio = (ratio - .25f);
    //			float size = (float) Math.exp(5 * subRatio);
    //
    //			g.pushTransform();
    //			g.translate(250, 250);
    //			g.scale(size, size);
    //			g.translate(-250, -250);
    //			g.setColor(new Color(1, 1, 1, 1 - ratio));
    //
    //			g.drawString(pip, 250 - pipWidth / 2, 250 - pipHeight);
    //			g.drawString(pop, 250 - popWidth / 2, 250);
    //
    //			g.popTransform();
    //		}

  }
}
