package com.pippop.managers;

import com.pippop.graph.Bubble;
import com.pippop.graph.Edge;
import com.pippop.graph.Graph;
import com.pippop.graph.Vertex;
import com.pippop.graphics.Color;
import com.pippop.graphics.Graphics;
import com.pippop.style.GameStyle;
import com.pippop.style.Style;
import com.pippop.util.Colors;
import com.pippop.util.RandomChooser;
import java.nio.FloatBuffer;
import java.util.Arrays;
import java.util.Collection;
import java.util.HashSet;
import java.util.List;
import java.util.Random;
import java.util.Set;

public class IntroSpawnManager {

  private static final int SPAWN_TIME = 500;

  private final Random RAND = new Random();
  private final RandomChooser<Color> colorChooser = Colors.getChooser(8);

  private int nextSpawnTime = 1000;

  public IntroSpawnManager() {}

  public void reset(Graph graph) {
    Color color1 = colorChooser.chooseRanom();
    Color color2 = colorChooser.chooseRanom(color1);
    double angle = RAND.nextFloat() * Math.PI;

    Style style1 = new IntroStyle(color1);
    Style style2 = new IntroStyle(color2);
    graph.reset(style1, style2, 0, 0, angle);
  }

  public void update(Graph graph, int delta) {
    nextSpawnTime -= delta;
    if (nextSpawnTime < 0) {
      spawn(graph);
      this.nextSpawnTime = SPAWN_TIME;
    }
  }

  private void spawn(Graph graph) {
    List<Vertex> vertices = graph.getVertices();
    Vertex vertex = vertices.get(RAND.nextInt(vertices.size()));
    spawn(graph, vertex);
  }

  private void spawn(Graph graph, Vertex vertex) {
    Color color = chooseColor(graph, vertex);
    Style style = new IntroStyle(color);
    graph.spawn(vertex, style);
  }

  private Color chooseColor(Graph graph, Vertex vertex) {
    Edge de1 = vertex.getEdge().getTwin();
    Edge de2 = de1.getNext().getTwin();
    Edge de3 = de2.getNext().getTwin();

    List<Bubble> bubbles = Arrays.asList(de1.getBubble(), de2.getBubble(), de3.getBubble());

    return colorChooser.chooseRanom(getColors(bubbles));
  }

  private Set<Color> getColors(Collection<Bubble> bubbles) {
    Set<Color> colors = new HashSet<>();
    for (Bubble bubble : bubbles) {
      if (bubble.getStyle() instanceof GameStyle) {
        GameStyle gameStyle = (GameStyle) bubble.getStyle();
        colors.add(gameStyle.getColor());
      }
    }
    return colors;
  }

  private static final class IntroStyle implements Style {
    private final Color color;

    private IntroStyle(Color color) {
      this.color = color;
    }

    @Override
    public void render(Graphics g, FloatBuffer shape, Color outlineColor) {
      //			g.setColor(color);
      //			g.fill(shape);
      //
      //			g.setLineWidth(2);
      //			g.setColor(outlineColor);
      //			g.draw(shape);
    }

    @Override
    public double getTargetArea() {
      return 3000;
    }
  }
}
