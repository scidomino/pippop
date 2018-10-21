package com.pippop;

import android.app.Activity;
import android.content.Intent;
import android.content.SharedPreferences;
import android.os.Bundle;
import android.os.CountDownTimer;
import android.view.View;
import android.widget.TextView;

public class GameOverActivity extends Activity {

  public static final String SCORE_PREF = "Score";
  public static final String HIGH_SCORE = "High";
  public static final String CURRENT_SCORE = "Current";

  @Override
  protected void onCreate(Bundle savedInstanceState) {
    super.onCreate(savedInstanceState);
    setContentView(R.layout.activity_game_over);

    SharedPreferences scorePrefs = getSharedPreferences(SCORE_PREF, MODE_PRIVATE);
    long highScore = scorePrefs.getLong(HIGH_SCORE, 0);
    long current = scorePrefs.getLong(CURRENT_SCORE, 0);

    TextView showCurrent = findViewById(R.id.showCurrent);
    showCurrent.setText(getBaseContext().getString(R.string.your_score, current));

    TextView showHigh = findViewById(R.id.highScore);
    showHigh.setText(getBaseContext().getString(R.string.high_score, highScore));

    if (highScore < current) {
      scorePrefs.edit().putLong(HIGH_SCORE, current).apply();
      showHigh.setText(getBaseContext().getString(R.string.new_high_score));
    } else {
      showHigh.setText(getBaseContext().getString(R.string.high_score, highScore));
    }

    new CountDownTimer(10000, 10000) {
      @Override
      public void onFinish() {
        backToMain(null);
        finish();
      }

      public void onTick(long millisUntilFinished) {}
    }.start();
  }

  public void backToMain(View view) {
    startActivity(new Intent(this, MainActivity.class));
  }
}
