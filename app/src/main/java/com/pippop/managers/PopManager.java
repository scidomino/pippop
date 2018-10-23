package com.pippop.managers;

import android.content.Context;
import android.media.MediaPlayer;
import com.pippop.R;
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
import com.pippop.util.PoppedBubble;
import java.nio.FloatBuffer;
import java.util.ArrayList;
import java.util.Iterator;
import java.util.List;
import java.util.Set;

public class PopManager {

  private static final int POPPING_TIME_MILLIS = 500;
  private static final int UNOTICEABLE_AREA = 10;

  private final List<Bubble> deflating = new ArrayList<>();
  private final MediaPlayer sound;
  private final BurstManager burst;
  private FloatBuffer popShape = Graphics.createVertexBuffer(200);
  private Bubble pending;
  private int pendingTime;
  private GameStyle pendingStyle;

  public PopManager(BurstManager burst, Context context) {
    this.burst = burst;
    this.sound = MediaPlayer.create(context, R.raw.pop);
  }

  public boolean deflateBigBubble(Graph graph) {
    for (Bubble bubble : graph.getBubbles()) {
      if (bubble.getStyle() instanceof GameStyle) {
        GameStyle gameStyle = (GameStyle) bubble.getStyle();
        if (gameStyle.isPoppable()) {
          pendingTime = POPPING_TIME_MILLIS;
          pending = bubble;
          this.pendingStyle = gameStyle;
          pending.setStyle(new EmptyStyle(this.pendingStyle.getTargetArea()));

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
      } else if (bubble.getArea() < UNOTICEABLE_AREA) {
        for (Edge edge : bubble) {
          Bubble other = edge.getTwin().getBubble();
          if (bubble.sharesExactlyOneEdge(other)) {
            graph.detach(edge.getTwin());
            it.remove();
            burst.burstAll(graph);
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
    PoppedBubble poppedBubble = new PoppedBubble(pending.getCenter(), pendingStyle);
    pending.setStyle(new EmptyStyle(0));
    pending = null;

    sound.seekTo(0);
    sound.start();
    return poppedBubble;
  }

  public boolean isDone() {
    return pendingTime < 0;
  }

  public boolean isPopping() {
    return pending != null;
  }

  public void render(Graphics g) {

    double radius = 5 * (Math.sqrt(pendingStyle.getTargetArea() / Math.PI));

    // closer to 0 closer to full circle
    float morphRatio = (float) Math.pow(pendingTime / (float) POPPING_TIME_MILLIS, 2);
    updatePopShape(radius, morphRatio);

    Color color = pendingStyle.getColor().withAlpha((1 + morphRatio) / 2);

    new GameStyle(pendingStyle.getPoint(), color).render(g, popShape, Color.WHITE);
  }

  private void updatePopShape(double radius, float morphRatio) {
    float invM = 1 - morphRatio;

    Point center = pending.getCenter();
    Vertex start = pending.getFirstEdge().getStart();
    double startAngle = Math.atan2(start.y - center.y, start.x - center.x);

    FloatBuffer buffer = pending.getBuffer();
    if (popShape.capacity() < buffer.limit()) {
      popShape = Graphics.createVertexBuffer(buffer.capacity());
    }
    popShape.clear();
    popShape.put(center.x);
    popShape.put(center.y);
    for (int i = 2; i < buffer.limit() - 2; i += 2) {
      double angle = startAngle - (2 * Math.PI) * (i / (float) buffer.limit());

      float circleX = center.x + (float) (Math.cos(angle) * radius);
      float circleY = center.y + (float) (Math.sin(angle) * radius);

      popShape.put(invM * circleX + morphRatio * buffer.get(i));
      popShape.put(invM * circleY + morphRatio * buffer.get(i + 1));
    }
    popShape.put(popShape.get(2));
    popShape.put(popShape.get(3));
    popShape.flip();
  }
}
