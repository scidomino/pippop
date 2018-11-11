package com.pippop.graph;

import com.pippop.graphics.Color;
import com.pippop.graphics.Graphics;
import com.pippop.style.Style;
import java.nio.BufferOverflowException;
import java.nio.FloatBuffer;
import java.util.HashSet;
import java.util.Iterator;
import java.util.Set;

/**
 * Represents a single bubble. It holds firstEdge which can be viewed as a circularly linked list of
 * the edges that make up the bubble.
 *
 * <p>User: Tommaso Sciortino Date: Oct 24, 2011 Time: 8:49:28 PM
 */
public class Bubble implements Iterable<Edge> {

  // First point is the center, then the polygon, the last point is the same as the second point.
  private FloatBuffer buffer;
  private Edge firstEdge;
  private Style style;
  // These are manually updated whenever the underlying edges change
  private double area;
  private Point center;

  public Bubble(Style style, Edge firstEdge) {
    this(style, firstEdge, Graphics.createFloatBuffer(200));
  }

  Bubble(Style style, Edge firstEdge, FloatBuffer buffer) {
    this.firstEdge = firstEdge;
    this.style = style;
    this.buffer = buffer;
    for (Edge edge : this) {
      edge.setBubble(this);
    }
    update();
  }

  private static boolean intersects(float x1, float y1, float x2, float y2, float x, float y) {
    if (y1 > y2) {
      return intersects(x2, y2, x1, y1, x, y);
    }

    if (y == y1 || y == y2) {
      y += 0.0001;
    }

    return !(y > y2)
        && !(y < y1)
        && !(x >= Math.max(x1, x2))
        && (x < Math.min(x1, x2) || (y - y1) / (x - x1) >= (y2 - y1) / (x2 - x1));
  }

  public double getPressureRatio() {
    return style.getTargetArea() / area;
  }

  public Set<Bubble> getAdjacentBubbles() {
    Set<Bubble> adjacents = new HashSet<>();
    for (Edge edge : this) {
      adjacents.add(edge.getTwin().getBubble());
    }
    return adjacents;
  }

  Set<Edge> getDegenerateEdges() {
    Set<Edge> degenerates = new HashSet<>();
    for (Edge edge : this) {
      if (edge.getTwin().getBubble() == this) {
        degenerates.add(edge);
      }
    }
    return degenerates;
  }

  public Style getStyle() {
    return style;
  }

  public void setStyle(Style style) {
    this.style = style;
  }

  public Edge getFirstEdge() {
    return firstEdge;
  }

  void setFirstEdge(Edge start) {
    this.firstEdge = start;
  }

  public void render(Graphics graphics, Color outline) {
    style.render(graphics, buffer, outline);
  }

  public double getArea() {
    return area;
  }

  public Point getCenter() {
    return center;
  }

  public void update() {
    this.area = calculateArea();
    this.center = calculateCenter();
    repopulateBuffer();
  }

  private void repopulateBuffer() {
    try {
      buffer.clear();
      buffer.put(center.x);
      buffer.put(center.y);
      for (Edge edge : this) {
        edge.flatten(buffer);
      }
      buffer.put(firstEdge.getStart().x);
      buffer.put(firstEdge.getStart().y);
      buffer.flip();
    } catch (BufferOverflowException e) {
      if (buffer.capacity() > 10000) {
        // Forget it, Jake. It's Chinatown.
        return;
      }
      buffer = Graphics.createFloatBuffer(2 * buffer.capacity());
      repopulateBuffer();
    }
  }

  public FloatBuffer getBuffer() {
    return buffer;
  }

  private double calculateArea() {
    double area = 0;
    for (Edge edge : this) {
      area += edge.getArea();
    }
    return Math.max(area, 1); // ensure always positive
  }

  private Point calculateCenter() {
    if (area == 0) {
      return this.firstEdge.getCenter();
    }

    float centroidX = 0;
    float centroidY = 0;
    for (Edge edge : this) {
      centroidX += edge.getCentroidComponentX();
      centroidY += edge.getCentroidComponentY();
    }

    float x = (float) (centroidX / area);
    float y = (float) (-centroidY / area);
    return new Point(x, y);
  }

  public boolean contains(Point point) {
    boolean inside = false;
    for (int i = 2; i < buffer.limit() - 2; i += 2) {
      float x1 = buffer.get(i);
      float y1 = buffer.get(i + 1);
      float x2 = buffer.get(i + 2);
      float y2 = buffer.get(i + 3);
      if (intersects(x1, y1, x2, y2, point.x, point.y)) {
        inside = !inside;
      }
    }
    return inside;
  }

  public boolean sharesExactlyOneEdge(Bubble o) {
    int count = 0;
    for (Edge edge : this) {
      if (edge.getTwin().getBubble() == o) {
        count++;
      }
    }
    return count == 1;
  }

  @Override
  public Iterator<Edge> iterator() {
    return new Iterator<Edge>() {

      private Edge current;

      @Override
      public boolean hasNext() {
        return current == null || current.getNext() != firstEdge;
      }

      @Override
      public Edge next() {
        if (current == null) {
          current = firstEdge;
        } else {
          current = current.getNext();
        }
        return current;
      }

      @Override
      public void remove() {
        throw new UnsupportedOperationException();
      }
    };
  }
}
