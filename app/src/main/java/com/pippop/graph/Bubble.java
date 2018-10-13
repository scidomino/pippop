package com.pippop.graph;

import static java.util.stream.Collectors.toSet;

import com.pippop.graphics.Color;
import com.pippop.graphics.Graphics;
import com.pippop.graphics.Polygon;
import com.pippop.style.Style;
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
  public final Polygon shape;
  private Edge firstEdge;
  private Style style;
  // These are manually updated whenever the underlying edges change
  private double area;
  private Point center;

  public Bubble(Style style, Edge start, Polygon polygon) {
    this.shape = polygon;
    this.firstEdge = start;
    this.style = style;
    stream().forEach(edge -> edge.setBubble(this));
    update();
  }

  public Bubble(Style style, Edge start) {
    this(style, start, new Polygon(100));
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
    style.render(graphics, shape, outline);
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

  public Polygon getShape() {
    return shape;
  }

  public void update() {
    this.area = calculateArea();
    this.center = calculateCenter();
    shape.update(this);
  }

  public double calculateArea() {
    return stream().mapToDouble(Edge::getArea).sum();
  }

  public Point calculateCenter() {
    if (area == 0) {
      return this.firstEdge.getCenter();
    }

    float x = 0;
    float y = 0;
    for (Edge edge : this) {
      x += edge.getCentroidComponentX();
      y += edge.getCentroidComponentY();
    }

    x /= area;
    y /= area;
    return new Point(x, -y);
  }

  public boolean contains(Point point) {
    return shape.contains(point.x, point.y);
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
