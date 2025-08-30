package com.pippop;

import android.app.Activity;
import android.os.Bundle;

public class ZenActivity extends Activity {

  private ZenView content;

  @Override
  protected void onCreate(Bundle savedInstanceState) {
    super.onCreate(savedInstanceState);
    setContentView(R.layout.activity_zen);
    content = findViewById(R.id.fullscreen_content);
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
