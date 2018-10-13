package com.pippop.util;

import android.content.Context;
import android.content.SharedPreferences;

public class ScoreBoard {
    public static final String PREFS_NAME = "CurrentScore";
  private static final int MAX_HIGH_SCORES = 5;
  Context context;

  private long currentScore;
  private GameStats levelStats = new GameStats();
  private GameStats gameStats = new GameStats();

  public ScoreBoard(Context context) {
    this.context = context;
  }

  public long getCurrentScore() {

    return currentScore;
  }

  public GameStats getLevelStats() {
    return levelStats;
  }

  public GameStats getGameStats() {
    return gameStats;
  }

  public void resetCurrentScore() {
    currentScore = 0;

    levelStats = new GameStats();
    gameStats = new GameStats();
  }

  public void addToCurrentScore(long points) {
    currentScore += points;
      SharedPreferences currentScore = context.getSharedPreferences(PREFS_NAME, Context
              .MODE_PRIVATE);
      currentScore.edit().putLong("CurrentScore", this.currentScore).apply();
  }
}
