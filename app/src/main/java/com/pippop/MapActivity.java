package com.pippop;

import android.app.Activity;
import android.content.Intent;
import android.os.Bundle;
import android.view.View;
import android.view.animation.AnimationUtils;
import android.widget.TextView;

public class MapActivity extends Activity {

  @Override
  protected void onCreate(Bundle savedInstanceState) {
    super.onCreate(savedInstanceState);
    setContentView(R.layout.activity_map);

    TextView chooseMode = findViewById(R.id.chooseMode);
    chooseMode.startAnimation(AnimationUtils.loadAnimation(this, R.anim.scale));
  }

  public void startZen(View view) {
    startActivity(new Intent(this, ZenActivity.class));
  }

  public void startOtterSwap(View view) {
    startActivity(new Intent(this, OtterSwapActivity.class));
  }
}
