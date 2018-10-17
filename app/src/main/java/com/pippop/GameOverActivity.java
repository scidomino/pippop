package com.pippop;

import android.app.Activity;
import android.content.Intent;
import android.os.Bundle;
import android.os.CountDownTimer;
import android.widget.TextView;

public class GameOverActivity extends Activity {

  private static final String PREFS_NAME = "LocalHighScore";
  private static final String PREFS_NAME2 = "CurrentScore";

  @Override
  protected void onCreate(Bundle savedInstanceState) {
    super.onCreate(savedInstanceState);
    setContentView(R.layout.activity_game_over);

    Long highscore = getSharedPreferences(PREFS_NAME, MODE_PRIVATE).getLong("highScore", 0);
    TextView showHigh = findViewById(R.id.showHigh);
    showHigh.setText(getBaseContext().getString(R.string.high_score, highscore));

    Long current = getSharedPreferences(PREFS_NAME2, MODE_PRIVATE).getLong("CurrentScore", 0);
    TextView showCurrent = findViewById(R.id.showCurrent);
    showCurrent.setText(getBaseContext().getString(R.string.your_score, current));

    if (highscore < current) {
      getSharedPreferences(PREFS_NAME, MODE_PRIVATE).edit().putLong("highScore", current).apply();
      TextView newHigh = findViewById(R.id.newHigh);
      newHigh.setText("New High Score!");
    }

    // Go back to main screen after 4 second count down
    // Add 'new high score!' with score display if/else
    // Add high score entry if/else if w/ name entry later
    new CountDownTimer(4000, 1000) {
      @Override
      public void onFinish() {
        startActivity(new Intent(GameOverActivity.this, MainActivity.class));
        finish();
      }

      public void onTick(long millisUntilFinished) {}
    }.start();
  }
}
