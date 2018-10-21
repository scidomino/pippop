package com.pippop.util;

import java.util.Arrays;
import java.util.Collection;
import java.util.HashMap;
import java.util.Map;
import java.util.Map.Entry;
import java.util.Random;

public class RandomChooser<T> {

  private final Random random = new Random();
  private final Map<T, Integer> map = new HashMap<>();

  public void add(T obj, int ratio) {
    map.put(obj, ratio);
  }

  @SafeVarargs
  public final T chooseRanom(T... excluded) {
    return chooseRanom(Arrays.asList(excluded));
  }

  public T chooseRanom(Collection<T> excluded) {
    int nextInt = random.nextInt(getTotal(excluded));
    for (Entry<T, Integer> entry : map.entrySet()) {
      if (excluded.contains(entry.getKey())) {
        continue;
      }
      nextInt -= entry.getValue();
      if (nextInt < 0) {
        return entry.getKey();
      }
    }
    return null;
  }

  private int getTotal(Collection<T> excluded) {
    int total = 0;
    for (Entry<T, Integer> e : map.entrySet()) {
      if (!excluded.contains(e.getKey())) {
        total += e.getValue();
      }
    }
    return total;
  }

  public int getSize() {
    return map.size();
  }
}
