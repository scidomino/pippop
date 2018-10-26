package com.pippop.graphics.gltext;

import android.opengl.Matrix;
import com.pippop.graphics.Color;
import com.pippop.graphics.program.BatchTextureProgram;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.nio.FloatBuffer;
import java.nio.ShortBuffer;

class SpriteBatch {

  private static final int VERTICES_PER_SPRITE = 4;
  private static final int INDICES_PER_SPRITE = 6;
  private static final int INDEX_SIZE = Short.SIZE / 8;

  private final FloatBuffer vertices;
  private final ShortBuffer indices;
  private final int maxSprites;

  private final BatchTextureProgram program;
  private final float[] uMVPMatrices;
  private int numSprites;

  SpriteBatch(int maxSprites, BatchTextureProgram program) {
    this.maxSprites = maxSprites;
    this.numSprites = 0;
    this.uMVPMatrices = new float[maxSprites * 16];

    this.vertices =
        ByteBuffer.allocateDirect(maxSprites * VERTICES_PER_SPRITE * 5 * 4)
            .order(ByteOrder.nativeOrder())
            .asFloatBuffer();
    indices = getIndices(maxSprites);

    this.program = program;
  }

  private ShortBuffer getIndices(int maxSprites) {
    ShortBuffer indices =
        ByteBuffer.allocateDirect(maxSprites * INDICES_PER_SPRITE * INDEX_SIZE)
            .order(ByteOrder.nativeOrder())
            .asShortBuffer();
    for (int i = 0; i < maxSprites * VERTICES_PER_SPRITE; i += VERTICES_PER_SPRITE) {
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

  void beginBatch(Color color, int textureId) {
    numSprites = 0;
    program.start(color, textureId);
  }

  void endBatch() {
    if (numSprites > 0) {
      vertices.flip();
      program.end(numSprites, uMVPMatrices, vertices, indices);
      vertices.clear();
    }
  }

  void drawSprite(
      float x,
      float y,
      float width,
      float height,
      TextureRegion region,
      float[] modelMatrix,
      float[] vpMatrix) {
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

    Matrix.multiplyMM(uMVPMatrices, numSprites * 16, vpMatrix, 0, modelMatrix, 0);

    numSprites++;
  }
}
