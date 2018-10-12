package com.pippop.util;

import android.content.Context;
import android.content.SharedPreferences;

public class ScoreBoard {
  private static final int MAX_HIGH_SCORES = 5;
  public static final String PREFS_NAME = "CurrentScore";

  Context context;

  private long currentScore;
  private GameStats levelStats = new GameStats();
  private GameStats gameStats = new GameStats();

  // private final List<HighScore> highScores = new ArrayList<HighScore>();

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
    SharedPreferences CurrentScore = context.getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE);
    SharedPreferences.Editor editor = CurrentScore.edit();
    editor.putLong("CurrentScore", currentScore);
    editor.commit();
    // ScorePoints.drawPoints(points);
  }
}

// STUFF TOMMASO MADE
//	public void compareScores() {
//		long current = getCurrentScore();
//		long previous = getLocalHighScore();
//        int dif = current.compareTo(previous);
//
//        if(dif > 0) {
//          updateLocalHighScore(current);
//        }
//	}

// public void updateLocalHighScore(){
//
// }

// public void registerHighScore(String name) {
//		highScores.add(new HighScore(name, currentScore));
//		truncateAndSort();
//		save();
//	}

//	private void truncateAndSort() {
//		Collections.sort(highScores);
//
//		while (highScores.size() > MAX_HIGH_SCORES) {
//			highScores.remove(highScores.size() - 1);
//		}
//	}
//
//	public boolean hasHighScore() {
//		return highScores.size() < MAX_HIGH_SCORES
//				|| highScores.get(highScores.size() - 1).getScore() < currentScore;
//	}

//	public void draw(Graphics g) {
//		String title = "High Scores!";
//		g.drawString(title, 250 - g.getFont().getWidth(title) / 2, 50);
//
//		int y = 100;
//		for (HighScore highScore : highScores) {
//			highScore.draw(g, y);
//			y += 50;
//		}
//	}

//	public void save() {
//		try {
//			SavedState state = new SavedState("highscores");
//			state.clear();
//			int i = 0;
//			for (HighScore hs : highScores) {
//				state.setString("name-" + i, hs.getName());
//				state.setNumber("score-" + i, hs.getScore());
//				i++;
//			}
//			state.save();
//		} catch (Exception e) {
//			System.err.println("could not save highscores: " + e.getMessage());
//		}
//	}

//	public void load() {
//		try {
//			SavedState state = new SavedState("highscores");
//			state.load();
//			for (int i = 0;; i++) {
//				String name = state.getString("name-" + i);
//				double score = state.getNumber("score-" + i);
//				if (name == null) {
//					break;
//				}
//				highScores.add(new HighScore(name, (long) score));
//			}
//			truncateAndSort();
//		} catch (Exception e) {
//			System.err.println("could not load highscores: " + e.getMessage());
//		}
//	}
