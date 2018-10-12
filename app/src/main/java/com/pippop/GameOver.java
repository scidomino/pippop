package com.pippop;

import android.app.Activity;
import android.content.Intent;
import android.content.SharedPreferences;
import android.os.Bundle;
import android.os.CountDownTimer;
import android.widget.TextView;

import com.pippop.graphics.CustomFont;

public class GameOver extends Activity {
  public static final String PREFS_NAME = "LocalHighScore";
  public static final String PREFS_NAME2 = "CurrentScore";

  @Override
  protected void onCreate(Bundle savedInstanceState) {
    super.onCreate(savedInstanceState);
    setContentView(R.layout.activity_game_over);

    // see if there is a new high score; update it if there is

    SharedPreferences LocalHighScore = getSharedPreferences(PREFS_NAME, MODE_PRIVATE);
    Long highscore = LocalHighScore.getLong("highScore", 0);
    String highStr = String.format("High score: %1$d", highscore);
    TextView showHigh = (TextView) findViewById(R.id.showHigh);
    showHigh.setText(highStr);
    showHigh.setTypeface(CustomFont.getTypeface(this));

    SharedPreferences CurrentScore = getSharedPreferences(PREFS_NAME2, MODE_PRIVATE);
    Long current = CurrentScore.getLong("CurrentScore", 0);
    String currentStr = String.format("Your score: %1$d", current);
    TextView showCurrent = (TextView) findViewById(R.id.showCurrent);
    showCurrent.setText(currentStr);
    showCurrent.setTypeface(CustomFont.getTypeface(this));

    TextView gameOver = (TextView) findViewById(R.id.gameOver);
    gameOver.setTypeface(CustomFont.getTypeface(this));

    if (highscore < current) {
      SharedPreferences.Editor editor = LocalHighScore.edit();
      editor.putLong("highScore", current);
      editor.commit();
      TextView newHigh = (TextView) findViewById(R.id.newHigh);
      newHigh.setTypeface(CustomFont.getTypeface(this));
      newHigh.setText("New High Score!");
    }

    // Go back to main screen after 4 second count down
    // Add 'new high score!' with score display if/else
    // Add high score entry if/else if w/ name entry later
    new CountDownTimer(4000, 1000) {
      @Override
      public void onFinish() {
        Intent returnToMain = new Intent(GameOver.this, MainActivity.class);
        startActivity(returnToMain);
        finish();
      }

      public void onTick(long millisUntilFinished) {}
    }.start();
  }

  @Override
  protected void onPause() {
    super.onPause();
  }

  @Override
  protected void onStart() {
    super.onStart();
  }

  @Override
  protected void onResume() {
    super.onResume();
  }
}
