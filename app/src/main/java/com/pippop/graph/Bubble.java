package com.pippop.graph;

import static java.util.stream.Collectors.toSet;

import com.pippop.graphics.Color;
import com.pippop.graphics.Graphics;
import com.pippop.graphics.PolygonCrossingsEvaluator;
import com.pippop.style.Style;
import java.nio.BufferOverflowException;
import java.nio.FloatBuffer;
import java.util.Iterator;
import java.util.Set;
import java.util.stream.Stream;
import java.util.stream.StreamSupport;

/**
 * Represents a single bubble. It holds firstEdge which can be viewed as a circularly linked list of
 * the edges that make up the bubble.
 *
 * <p>User: Tommaso Sciortino Date: Oct 24, 2011 Time: 8:49:28 PM
 */
public class Bubble implements Iterable<Edge> {

  // First point is the center, the last point is the same as the second point.
  private FloatBuffer buffer;
  private Edge firstEdge;
  private Style style;
  // These are manually updated whenever the underlying edges change
  private double area;
  private Point center;

  public Bubble(Style style, Edge firstEdge) {
    this(style, firstEdge, Graphics.createVertexBuffer(100));
  }

  Bubble(Style style, Edge firstEdge, FloatBuffer buffer) {
    this.firstEdge = firstEdge;
    this.style = style;
    this.buffer = buffer;
    stream().forEach(edge -> edge.setBubble(this));
    update();
  }

  private static boolean isBelow(Point start, Point end, Point point) {
    float endX = end.x - start.x;
    float endY = end.y - start.y;
    float pointX = point.x - start.x;
    float pointY = point.y - start.y;
    return endX * pointY < endY * pointX;
  }

  public double getPressureRatio(double speedBump) {
    if (area * speedBump < style.getTargetArea()) {
      return -speedBump;
    }
    return -style.getTargetArea() / area;
  }

  public Set<Bubble> getAdjacentBubbles() {
    return stream().map(edge -> edge.getTwin().getBubble()).collect(toSet());
  }

  public Set<Edge> getDegenerateEdges() {
    return stream().filter(edge -> edge.getTwin().getBubble() == this).collect(toSet());
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

  public void setFirstEdge(Edge start) {
    this.firstEdge = start;
  }

  public void render(Graphics graphics, Color outline) {
    style.render(graphics, buffer, outline);
  }

  public boolean isDeflating() {
    return style.getTargetArea() == 0;
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
      buffer.put(buffer.get(2));
      buffer.put(buffer.get(3));
      buffer.flip();
    } catch (BufferOverflowException e) {
      if (buffer.capacity() > 10000) {
        // Forget it, Jake. It's Chinatown.
        return;
      }
      buffer = Graphics.createVertexBuffer(2 * buffer.capacity());
      repopulateBuffer();
    }
  }

  public FloatBuffer getBuffer() {
    return buffer;
  }

  private double calculateArea() {
    return stream().mapToDouble(Edge::getArea).sum();
  }

  private Point calculateCenter() {
    if (area == 0) {
      return this.firstEdge.getCenter();
    }

    float x = (float) (stream().mapToDouble(Edge::getCentroidComponentX).sum() / area);
    float y = (float) (-stream().mapToDouble(Edge::getCentroidComponentY).sum() / area);
    return new Point(x, y);
  }

  public boolean contains(Point point) {
    return PolygonCrossingsEvaluator.contains(buffer, point.x, point.y);
  }

  public boolean sharesExactlyOneEdge(Bubble o) {
    return stream().filter(edge -> edge.getTwin().getBubble() == o).count() == 1;
  }

  public Edge getCorrespondingEdge(Point point) {
    Point center = getCenter();
    boolean prevBelow = isBelow(center, firstEdge.getStart(), point);
    for (Edge edge : this) {
      boolean below = isBelow(center, edge.getEnd(), point);
      if (prevBelow && !below) {
        return edge;
      }
      prevBelow = below;
    }
    return null;
  }

  public Stream<Edge> stream() {
    return StreamSupport.stream(spliterator(), false);
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
