package com.pippop;

import android.app.Activity;
import android.opengl.GLSurfaceView;
import android.os.Bundle;
import android.widget.TextView;

public class GameActivity extends Activity {
  private GLSurfaceView content;

  @Override
  protected void onCreate(Bundle savedInstanceState) {
    super.onCreate(savedInstanceState);

    setContentView(R.layout.activity_game);
      content = findViewById(R.id.fullscreen_content);
      TextView derp = findViewById(R.id.showCurrent);
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
