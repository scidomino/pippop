package com.pippop;

import android.app.Activity;
import android.opengl.GLSurfaceView;
import android.os.Bundle;

// import com.pippop.graphics.Text.Glyph;

public class HighScoreActivity extends Activity {
  private GLSurfaceView glView;

  // this activity is for updating the GLOBAL high scores hosted online
  @Override
  protected void onCreate(Bundle savedInstanceState) {
    super.onCreate(savedInstanceState);
    setContentView(R.layout.activity_high_score);
    // glView = new GLSurf( this );
    // setContentView( glView );

  }

  @Override
  protected void onPause() {
    super.onPause();
    // The following call pauses the rendering thread.
    // If your OpenGL application is memory intensive,
    // you should consider de-allocating objects that
    // consume significant memory here.
    glView.onPause();
  }

  @Override
  protected void onResume() {
    super.onResume();
    // The following call resumes a paused rendering thread.
    // If you de-allocated graphic objects for onPause()
    // this is a good place to re-allocate them.
    glView.onResume();
  }
}

//    class TexampleSurfaceView extends GLSurfaceView {
//
//        public TexampleSurfaceView(Context context){
//            super( context );

            // Set to use OpenGL ES 2.0
            // setEGLContextClientVersion(2);

            // Set the Renderer for drawing on the GLSurfaceView
            // setRenderer( new Glyph.GlyphRenderer( context ) );
//        }
//    }
