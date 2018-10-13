package com.pippop.managers;

import com.pippop.graph.Bubble;
import com.pippop.graph.Edge;
import com.pippop.graph.Graph;
import com.pippop.graph.OpenAir;
import com.pippop.graph.Point;
import com.pippop.graph.Vertex;
import com.pippop.graphics.Color;
import com.pippop.graphics.Graphics;
import com.pippop.graphics.Polygon;
import com.pippop.style.EmptyStyle;
import com.pippop.style.GameStyle;
import com.pippop.util.MorphShape;
import java.nio.FloatBuffer;
import java.util.ArrayList;
import java.util.Iterator;
import java.util.List;
import java.util.Set;

public class PopManager extends GraphManager {

  private static final int FREEZE_MILLISECONDS = 500;
  private static final int UNOTICEABLE_SIZE = 10;

  private final List<Bubble> deflating = new ArrayList<>();
  private final Polygon circle = new Polygon(100);
  private final MorphShape popShape = new MorphShape();
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

    FloatBuffer vertices = circle.getVertices();
    vertices.clear();
    for (int i = 0; i < 40; i++) {
      double angle = startAngle + (2 * Math.PI) * (i / 40f);
      vertices.put((float) (center.x + Math.cos(angle) * radius));
      vertices.put((float) (center.y + Math.sin(angle) * radius));
    }
    vertices.put(vertices.get(2));
    vertices.put(vertices.get(3));
    vertices.flip();

    popShape.build(circle, pending.getShape(), percentDone);
    gameStyle.render(g, popShape, Color.WHITE);
  }
}
