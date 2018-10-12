package com.pippop;

import android.app.Activity;
import android.content.Intent;
import android.content.SharedPreferences;
import android.os.Bundle;
import android.view.View;
import android.view.animation.Animation;
import android.view.animation.AnimationUtils;
import android.widget.Button;
import android.widget.TextView;

import com.pippop.graphics.CustomFont;

public class MainActivity extends Activity {
  public static final String PREFS_NAME = "LocalHighScore";

  @Override
  protected void onCreate(Bundle savedInstanceState) {
    super.onCreate(savedInstanceState);
    setContentView(R.layout.activity_main);

    SharedPreferences LocalHighScore = getSharedPreferences(PREFS_NAME, MODE_PRIVATE);
    Long score = LocalHighScore.getLong("highScore", 0);
    String highStr = String.format("High score: %1$d", score);

    TextView showHigh = (TextView) findViewById(R.id.showHigh);
    showHigh.setTypeface(CustomFont.getTypeface(this));
    showHigh.setText(highStr);

    TextView appName = (TextView) findViewById(R.id.appName);
    appName.setTypeface(CustomFont.getTypeface(this));
    //        Animation grow = AnimationUtils.loadAnimation(this, R.anim.);
    //        appName.startAnimation(grow);

    Button play = (Button) findViewById(R.id.play);
    play.setTypeface(CustomFont.getTypeface(this));
    Animation scale = AnimationUtils.loadAnimation(this, R.anim.scale);
    play.startAnimation(scale);
  }

  // Start GameActivity when Play is clicked
  public void startPlay(View view) {
    Intent startPlay = new Intent(this, GameActivity.class);
    startActivity(startPlay);
    //        Intent startPlay = new Intent(this, GameOver.class);
    //        startActivity(startPlay);
  }

  // Go to high score view when button clicked
  public void viewScores(View view) {
    setContentView(R.layout.activity_high_score);
  }

  @Override
  protected void onStart() {
    super.onStart();
    // The activity is about to become visible.
  }

  @Override
  protected void onResume() {
    super.onResume();
    // The activity has become visible (it is now "resumed").
  }

  @Override
  protected void onPause() {
    super.onPause();
    // Another activity is taking focus (this activity is about to be "paused").
  }

  @Override
  protected void onStop() {
    super.onStop();
    // The activity is no longer visible (it is now "stopped")
    // We need an Editor object to make preference changes.
    // All objects are from android.context.Context
    //        SharedPreferences settings = getSharedPreferences(PREFS_NAME, 0);
    //        SharedPreferences.Editor editor = settings.edit();
    //        editor.putString("localHighScore", "test2");

    //        // Commit the edits!
    //        editor.commit();

  }

  @Override
  protected void onDestroy() {
    super.onDestroy();
    // The activity is about to be destroyed.
  }
}
