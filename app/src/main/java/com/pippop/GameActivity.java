package com.pippop;

import android.app.Activity;
import android.content.Context;
import android.content.SharedPreferences;
import android.os.Bundle;
import android.widget.TextView;

public class GameActivity extends Activity {

  private GameView content;
  private TextView scoreBoard;
  private final SharedPreferences.OnSharedPreferenceChangeListener currentScore =
      (s, k) -> {
        long score = s.getLong("CurrentScore", 0);
        scoreBoard.setText(getBaseContext().getString(R.string.score, score));
      };

  @Override
  protected void onCreate(Bundle savedInstanceState) {
    super.onCreate(savedInstanceState);

    setContentView(R.layout.activity_game);
    content = findViewById(R.id.fullscreen_content);
    scoreBoard = findViewById(R.id.score_board);
    scoreBoard.setText(getBaseContext().getString(R.string.score, 0));

    getBaseContext()
        .getSharedPreferences("CurrentScore", Context.MODE_PRIVATE)
        .registerOnSharedPreferenceChangeListener(currentScore);
  }

  @Override
  protected void onPause() {
    super.onPause();
    content.onPause();
  }

  @Override
  protected void onResume() {
    super.onResume();
    content.onResume();
  }
}
