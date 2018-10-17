package com.pippop.managers;

import com.pippop.graph.Bubble;
import com.pippop.graph.Edge;
import com.pippop.graph.Graph;
import com.pippop.graph.OpenAir;
import com.pippop.graph.Point;
import com.pippop.graph.Vertex;
import com.pippop.graphics.Color;
import com.pippop.graphics.Graphics;
import com.pippop.style.EmptyStyle;
import com.pippop.style.GameStyle;
import com.pippop.util.Tweener;
import java.nio.FloatBuffer;
import java.util.ArrayList;
import java.util.Iterator;
import java.util.List;
import java.util.Set;

public class PopManager extends GraphManager {

  private static final int FREEZE_MILLISECONDS = 500;
  private static final int UNOTICEABLE_SIZE = 10;

  private final List<Bubble> deflating = new ArrayList<>();
  private final FloatBuffer circle = Graphics.createVertexBuffer(100);
  private final FloatBuffer popShape = Graphics.createVertexBuffer(100);
  private Bubble pending;
  private int pendingTime;

  public boolean deflateBigBubble(Graph graph) {
    for (Bubble bubble : graph.getBubbles()) {
      if (bubble.getStyle() instanceof GameStyle) {
        GameStyle gameStyle = (GameStyle) bubble.getStyle();
        if (gameStyle.isPoppable()) {
          pendingTime = FREEZE_MILLISECONDS;
          pending = bubble;
          return true;
        }
      }
    }
    return false;
  }

  public void removeDeflated(Graph graph) {
    OpenAir openAir = graph.getOpenAir();
    Set<Bubble> touchingAir = openAir.getAdjacentBubbles();
    Iterator<Bubble> it = deflating.iterator();
    while (it.hasNext()) {
      Bubble bubble = it.next();
      if (touchingAir.contains(bubble)) {
        for (Edge edge : bubble) {
          if (edge.getTwin().getBubble() == openAir) {
            graph.detach(edge.getTwin());
            it.remove();
            break;
          }
        }
      } else if (bubble.getArea() < UNOTICEABLE_SIZE) {
        for (Edge edge : bubble) {
          Bubble other = edge.getTwin().getBubble();
          if (bubble.sharesExactlyOneEdge(other)) {
            graph.detach(edge.getTwin());
            it.remove();
            burstAll(graph);
            break;
          }
        }
      }
    }
  }

  public void update(int delta) {
    pendingTime -= delta;
  }

  public PoppedBubble popBubble() {
    deflating.add(pending);
    PoppedBubble poppedBubble =
        new PoppedBubble(pending.getCenter(), (GameStyle) pending.getStyle());
    pending.setStyle(new EmptyStyle());
    pending = null;
    return poppedBubble;
  }

  public boolean isDone() {
    return pendingTime < 0;
  }

  public boolean isPopping() {
    return pending != null;
  }

  public void render(Graphics g) {
    float percentDone = pendingTime / (float) FREEZE_MILLISECONDS;
    percentDone = percentDone * percentDone;

    Point center = pending.getCenter();
    GameStyle gameStyle = (GameStyle) pending.getStyle();
    int radius = 5 * (int) (Math.sqrt(gameStyle.getTargetArea() / Math.PI));

    Vertex start = pending.getFirstEdge().getStart();
    double startAngle = Math.atan2(start.y - center.y, start.x - center.x);

    populateCircle(center, radius, startAngle);

    Tweener.tween(circle, pending.getBuffer(), popShape, percentDone);
    gameStyle.render(g, popShape, Color.WHITE);
  }

  private void populateCircle(Point center, int radius, double startAngle) {
    circle.clear();
    for (int i = 0; i < 40; i++) {
      double angle = startAngle + (2 * Math.PI) * (i / 40f);
      circle.put((float) (center.x + Math.cos(angle) * radius));
      circle.put((float) (center.y + Math.sin(angle) * radius));
    }
    circle.put(circle.get(2));
    circle.put(circle.get(3));
    circle.flip();
  }
}
