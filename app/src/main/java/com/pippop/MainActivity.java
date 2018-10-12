package com.pippop;

import android.app.Activity;
import android.content.Intent;
import android.content.SharedPreferences;
import android.os.Bundle;
import android.view.View;
import android.view.animation.AnimationUtils;
import android.widget.Button;
import android.widget.TextView;

public class MainActivity extends Activity {
  public static final String PREFS_NAME = "LocalHighScore";

  @Override
  protected void onCreate(Bundle savedInstanceState) {
    super.onCreate(savedInstanceState);
    setContentView(R.layout.activity_main);

    SharedPreferences LocalHighScore = getSharedPreferences(PREFS_NAME, MODE_PRIVATE);
    Long score = LocalHighScore.getLong("highScore", 0);
    String highStr = String.format("High score: %1$d", score);

      TextView showHigh = findViewById(R.id.showHigh);
    showHigh.setText(highStr);

      Button play = findViewById(R.id.play);
      play.startAnimation(AnimationUtils.loadAnimation(this, R.anim.scale));
  }

  public void startPlay(View view) {
      startActivity(new Intent(this, GameActivity.class));
  }

  public void viewScores(View view) {
    setContentView(R.layout.activity_high_score);
  }

  @Override
  protected void onStart() {
    super.onStart();
  }

  @Override
  protected void onResume() {
    super.onResume();
  }

  @Override
  protected void onPause() {
    super.onPause();
  }

  @Override
  protected void onStop() {
    super.onStop();
  }

  @Override
  protected void onDestroy() {
    super.onDestroy();
  }
}
