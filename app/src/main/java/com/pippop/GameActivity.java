package com.pippop;

import android.app.Activity;
import android.opengl.GLSurfaceView;
import android.os.Bundle;

public class GameActivity extends Activity {
  private GLSurfaceView content;

  @Override
  protected void onCreate(Bundle savedInstanceState) {
    super.onCreate(savedInstanceState);

    setContentView(R.layout.activity_game);
    content = (GLSurfaceView) findViewById(R.id.fullscreen_content);
  }

  @Override
  protected void onPause() {
    super.onPause();
    content.onPause();
  }

  @Override
  protected void onStart() {
    super.onStart();
  }

  @Override
  protected void onResume() {
    super.onResume();
    content.onResume();
  }

  @Override
  public void onDestroy() {
    super.onDestroy();
  }

  @Override
  public void onStop() {
    super.onStop();
  }

  @Override
  public void onRestart() {
    super.onRestart();
  }
}
