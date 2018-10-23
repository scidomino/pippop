package com.pippop.graphics.gltext;


public enum AttribVariable {
  A_Position(1, "a_Position"),
  A_TexCoordinate(2, "a_TexCoordinate"),
  A_MVPMatrixIndex(3, "a_MVPMatrixIndex");

  private final int mHandle;
  private final String mName;

  AttribVariable(int handle, String name) {
    mHandle = handle;
    mName = name;
  }

  public int getHandle() {
    return mHandle;
  }

  public String getName() {
    return mName;
  }
}
