package com.pippop.graphics;

import android.content.Context;
import android.graphics.Typeface;

/** Created by L on 9/28/2016. Used Stack Overflow answer by Daniel L. */
public class CustomFont {

  private static boolean fontLoaded = false;
  private static Typeface font;
  private static String fontPath = "fonts/Sniglet-ExtraBold.ttf";

  public static Typeface getTypeface(Context context) {
    if (!fontLoaded) {
      loadFont(context);
    }

    return font;
  }

  private static void loadFont(Context context) {
    font = Typeface.createFromAsset(context.getAssets(), fontPath);
    fontLoaded = true;
  }
}
