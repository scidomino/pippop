package com.pippop.managers;

import android.content.Context;
import android.os.Environment;

import com.pippop.graph.Bubble;
import com.pippop.graph.Edge;
import com.pippop.graph.Graph;
import com.pippop.graph.OpenAir;
import com.pippop.graph.Variable;
import com.pippop.graph.Vertex;
import com.pippop.graphics.Color;
import com.pippop.style.GameStyle;
import com.pippop.style.Style;

import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;
import java.io.PrintWriter;
import java.text.SimpleDateFormat;
import java.util.Date;

public class SvgManager {

  public void write(Graph graph, Context context) {
    try {
      File dir =
          new File(
              Environment.getExternalStoragePublicDirectory(Environment.DIRECTORY_PICTURES),
              "pippop");
      dir.mkdirs();
      String time = new SimpleDateFormat("yyyy.MM.dd-HH.mm.ss").format(new Date());
      FileOutputStream out = new FileOutputStream(dir.getAbsoluteFile() + "/test." + time + ".svg");
      PrintWriter writer = new PrintWriter(out);
      try {
        writer.append(
            "<svg width=\"20cm\" height=\"20cm\" viewBox=\"-250 -250 500 500\" "
                + "xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\">\n");
        for (Bubble bubble : graph.getBubbles()) {
          if (bubble instanceof OpenAir) {
            continue;
          }
          writer.append("  <path stroke=\"black\" stroke-width=\"3\" ");
          writer.append("fill=\"" + getColorString(bubble) + "\" ");
          Vertex start = bubble.getFirstEdge().getStart();
          writer.append("d=\"M" + start.x + "," + start.y + " ");
          for (Edge edge : bubble) {
            Variable startCtrl = edge.getStartCtrl();
            Variable endCtrl = edge.getEndCtrl();
            Vertex end = edge.getEnd();
            writer.append("C" + startCtrl.x + "," + startCtrl.y + " ");
            writer.append(endCtrl.x + "," + endCtrl.y + " ");
            writer.append(end.x + "," + end.y + " ");
          }
          writer.append("\" />\n");
        }
        writer.append("</svg>\n");
      } finally {
        writer.close();
      }
    } catch (IOException e) {
      throw new RuntimeException(e);
    }
  }

  private String getColorString(Bubble bubble) {
    Style style = bubble.getStyle();
    if (style instanceof GameStyle) {
      GameStyle gameStyle = (GameStyle) style;
      Color color = gameStyle.getColor();
      int r = (int) (255 * color.getRed());
      int g = (int) (255 * color.getGreen());
      int b = (int) (255 * color.getBlue());
      String col = Integer.toHexString((r << 16) + (g << 8) + b);
      return "#" + ("000000".substring(col.length()) + col);
    } else {
      return "black";
    }
  }
}
