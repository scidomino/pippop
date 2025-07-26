package com.pippop;

import android.app.Activity;
import android.content.Intent;
import android.os.Bundle;
import android.view.View;
import android.view.animation.AnimationUtils;
import android.widget.TextView;

public class MainActivity extends Activity {

  private static final String PREFS_NAME = "LocalHighScore";

  @Override
  protected void onCreate(Bundle savedInstanceState) {
    super.onCreate(savedInstanceState);
    setContentView(R.layout.activity_main);

    Long highscore = getSharedPreferences(PREFS_NAME, MODE_PRIVATE).getLong("highScore", 0);
    TextView showHigh = findViewById(R.id.highScore);
    showHigh.setText(getBaseContext().getString(R.string.high_score, highscore));

    TextView play = findViewById(R.id.play);
    play.startAnimation(AnimationUtils.loadAnimation(this, R.anim.scale));
  }

  public void startPlay(View view) {
    startActivity(new Intent(this, MapActivity.class));
  }
}
