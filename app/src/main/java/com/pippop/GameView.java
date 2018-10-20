package com.pippop;

import android.annotation.SuppressLint;
import android.content.Context;
import android.content.Intent;
import android.opengl.GLES20;
import android.opengl.GLSurfaceView;
import android.util.AttributeSet;
import android.view.MotionEvent;
import com.pippop.graph.Graph;
import com.pippop.graph.Point;
import com.pippop.graphics.Graphics;
import com.pippop.managers.BlowoutManager;
import com.pippop.managers.BurstManager;
import com.pippop.managers.HighlightManager;
import com.pippop.managers.ImpossibleSuccessManager;
import com.pippop.managers.PipPopManager;
import com.pippop.managers.PopManager;
import com.pippop.managers.RandomSpawnManager;
import com.pippop.managers.ScoreManager;
import com.pippop.managers.ShowAndMoveManager;
import com.pippop.managers.SlideManager;
import com.pippop.managers.SpawnManager;
import com.pippop.managers.SuccessManager;
import com.pippop.managers.SwapManager;
import com.pippop.util.Colors;
import com.pippop.util.PoppedBubble;
import javax.microedition.khronos.egl.EGLConfig;
import javax.microedition.khronos.opengles.GL10;

public class GameView extends GLSurfaceView {
  private final Graph graph = new Graph();
  private final BurstManager burst = new BurstManager(getContext(), 1);
  private final HighlightManager highlight = new HighlightManager();
  private final ShowAndMoveManager showAndMove = new ShowAndMoveManager();
  private final PopManager pop = new PopManager(burst, getContext());
  private final SlideManager slide = new SlideManager(burst);
  private final SwapManager swap = new SwapManager();
  private final SpawnManager spawn = new RandomSpawnManager(Colors.getChooser(6), 20, getContext());
  private final PipPopManager pipPop = new PipPopManager();
  private final BlowoutManager blowout = new BlowoutManager();
  private final SuccessManager success = new ImpossibleSuccessManager();
  private final ScoreManager score = new ScoreManager();
  private State state = State.NORMAL;
  private Graphics graphics;

  public GameView(Context context, AttributeSet attrs) {
    super(context, attrs);
    setEGLContextClientVersion(2);
    setRenderer(new GameRenderer());
  }

  @SuppressLint("ClickableViewAccessibility")
  @Override
  public boolean onTouchEvent(MotionEvent event) {
    switch (event.getActionMasked()) {
      case MotionEvent.ACTION_DOWN:
        swap(new Point(event.getX(), event.getY()));
        break;
    }
    return true;
  }

  private void swap(final Point point) {
    queueEvent(
        () -> {
          if (state == State.NORMAL) {
            graphics.convertToBubbleSpacePoint(point);
            if (swap.swap(graph, point)) {
              state = State.SWAPPING;
            }
          }
        });
  }

  private enum State {
    NORMAL,
    SWAPPING,
    BURST,
    POPPING,
    PIPPOP
  }

  private class GameRenderer implements Renderer {
    private static final long MILIS_PER_FRAME = 1000 / 60;
    private long startTime = System.currentTimeMillis();

    GameRenderer() {
      spawn.reset(graph);
      success.reset();
      blowout.reset();
    }

    @Override
    public void onSurfaceCreated(GL10 gl, EGLConfig config) {
      GLES20.glClearColor(0.0f, 0.0f, 0.0f, 1.0f);
      graphics = new Graphics(getContext());
    }

    public void onSurfaceChanged(GL10 gl, int w, int h) {
      // makes adjustments for screen ratio
      graphics.updateDimensions(w, h);
    }

    public void onDrawFrame(GL10 gl) {
      long elapsed = Math.min(System.currentTimeMillis() - startTime, 1000);
      long dt = elapsed - MILIS_PER_FRAME;
      try {
        if (dt > 0) {
          Thread.sleep(dt);
        }
      } catch (InterruptedException e) {
        throw new IllegalStateException(e);
      }

      startTime = System.currentTimeMillis();
      update((int) elapsed);
      render(graphics);
    }

    void update(int delta) {
      showAndMove.update(graph, 0, 0);
      spawn.update(graph, delta);
      score.update(delta);
      pop.removeDeflated(graph);
      switch (state) {
        case NORMAL:
          slide.slideSlidableEdges(graph, delta);
          spawn.possiblySpawn(graph);

          if (pop.deflateBigBubble(graph)) {
            state = State.POPPING;
          } else {
            blowout.update(graph, delta);

            if (success.hasSucceeded(graph)) {
              if (!score.isProcessing()) {
                highlight.killHighlight();
              }
            } else if (blowout.isGameOver()) {
              highlight.killHighlight();
              Intent gameOverIntent = new Intent(getContext(), GameOverActivity.class);
              getContext().startActivity(gameOverIntent);
            }
          }

          break;

        case POPPING:
          pop.update(delta);
          if (pop.isDone()) {
            PoppedBubble popped = pop.popBubble();
            score.onPop(popped);
            success.onPop(popped);
            state = State.NORMAL;
          }
          break;
        case SWAPPING:
          swap.update(graph, delta);
          if (swap.isDone()) {
            spawn.swapDone();
            if (burst.findAndSetBurstableEdges(graph)) {
              state = State.BURST;
            } else {
              state = State.NORMAL;
            }
          }
          break;
        case BURST:
          burst.update(delta);
          if (burst.isDone()) {
            score.onBurst(burst.burstEdge(graph));
            if (!burst.isDone()) {
              state = State.BURST;
            } else if (burst.findAndSetBurstableEdges(graph)) {
              state = State.BURST;
            } else if (pipPop.hasPipPop(graph)) {
              pipPop.reset();
              state = State.PIPPOP;
            } else {
              state = State.NORMAL;
            }
          }
          break;
        case PIPPOP:
          pipPop.update(delta);
          if (pipPop.isDone()) {
            state = State.NORMAL;
          }
          break;
      }
    }

    void render(Graphics g) {
      GLES20.glClear(GLES20.GL_COLOR_BUFFER_BIT);
      showAndMove.render(g, graph, blowout.getColor());
      switch (state) {
        case POPPING:
          pop.render(g);
          break;
        case SWAPPING:
          swap.render(g);
          break;
        case BURST:
          burst.render(g);
          break;
        case PIPPOP:
          pipPop.render(g);
          break;
        case NORMAL:
          highlight.render(graph, g);
      }

      score.render(g);
    }
  }
}
