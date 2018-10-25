package com.pippop.graphics.gltext;

import android.opengl.GLES20;
import android.opengl.Matrix;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.nio.FloatBuffer;
import java.nio.ShortBuffer;

class SpriteBatch {

  // --Constants--//
  private static final int VERTEX_SIZE = 5;
  private static final int VERTICES_PER_SPRITE = 4; // Vertices Per Sprite
  private static final int INDICES_PER_SPRITE = 6; // Indices Per Sprite
  private static final int INDEX_SIZE = Short.SIZE / 8; // Index Byte Size (Short.SIZE = bits)
  private final FloatBuffer vertices;
  // --Members--//
  private Vertices oldVertex; // Vertices Instance Used for Rendering
  private int maxSprites; // Maximum Sprites Allowed in Buffer
  private int numSprites; // Number of Sprites Currently in Buffer
  private float[] mVPMatrix; // View and projection matrix specified at begin
  private float[] uMVPMatrices =
      new float[GLText.CHAR_BATCH_SIZE * 16]; // MVP matrix array to pass to shader
  private int mMVPMatricesHandle; // shader handle of the MVP matrix array
  private float[] mMVPMatrix = new float[16]; // used to calculate MVP matrix of each sprite

  // --Constructor--//
  // D: prepare the sprite batcher for specified maximum number of sprites
  // A: maxSprites - the maximum allowed sprites per batch
  //    program - program to use when drawing
  SpriteBatch(int maxSprites, int program) {
    this.maxSprites = maxSprites;
    this.numSprites = 0;

    this.vertices =
        ByteBuffer.allocateDirect(maxSprites * VERTICES_PER_SPRITE * VERTEX_SIZE * 4)
            .order(ByteOrder.nativeOrder())
            .asFloatBuffer();
    ShortBuffer indices = getIndices(maxSprites);

    this.oldVertex = new Vertices(indices);

    mMVPMatricesHandle = GLES20.glGetUniformLocation(program, "u_MVPMatrix");
  }

  private ShortBuffer getIndices(int maxSprites) {
    ShortBuffer indices =
        ByteBuffer.allocateDirect(maxSprites * INDICES_PER_SPRITE * INDEX_SIZE)
            .order(ByteOrder.nativeOrder())
            .asShortBuffer();
    for (int i = 0; i < maxSprites; i += VERTICES_PER_SPRITE) {
      indices.put((short) i);
      indices.put((short) (i + 1));
      indices.put((short) (i + 2));
      indices.put((short) (i + 2));
      indices.put((short) (i + 3));
      indices.put((short) i);
    }
    indices.flip();
    return indices;
  }

  void beginBatch(float[] vpMatrix) {
    numSprites = 0; // Empty Sprite Counter
    mVPMatrix = vpMatrix;
  }

  void endBatch() {
    if (numSprites > 0) {
      GLES20.glUniformMatrix4fv(mMVPMatricesHandle, numSprites, false, uMVPMatrices, 0);
      GLES20.glEnableVertexAttribArray(mMVPMatricesHandle);
      vertices.flip();
      oldVertex.bind(vertices);
      oldVertex.draw(numSprites * INDICES_PER_SPRITE);
      oldVertex.unbind();
      vertices.clear();
    }
  }

  void drawSprite(
      float x, float y, float width, float height, TextureRegion region, float[] modelMatrix) {
    if (numSprites == maxSprites) {
      endBatch();
      numSprites = 0;
    }

    float halfWidth = width / 2.0f;
    float halfHeight = height / 2.0f;
    float x1 = x - halfWidth;
    float y1 = y - halfHeight;
    float x2 = x + halfWidth;
    float y2 = y + halfHeight;

    vertices.put(x1).put(y1).put(region.u1).put(region.v2).put(numSprites);
    vertices.put(x2).put(y1).put(region.u2).put(region.v2).put(numSprites);
    vertices.put(x2).put(y2).put(region.u2).put(region.v1).put(numSprites);
    vertices.put(x1).put(y2).put(region.u1).put(region.v1).put(numSprites);

    Matrix.multiplyMM(mMVPMatrix, 0, mVPMatrix, 0, modelMatrix, 0);
    System.arraycopy(mMVPMatrix, 0, uMVPMatrices, numSprites * 16, 16);

    numSprites++;
  }
}
