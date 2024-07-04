package com.pippop.graphics.gltext;

class TextureRegion {

  final float u1;
  final float v1; // Top/Left U,V Coordinates
  final float u2;
  final float v2; // Bottom/Right U,V Coordinates

  TextureRegion(float texWidth, float texHeight, float x, float y, float width, float height) {
    this.u1 = x / texWidth; // Calculate U1
    this.v1 = y / texHeight; // Calculate V1
    this.u2 = this.u1 + (width / texWidth); // Calculate U2
    this.v2 = this.v1 + (height / texHeight); // Calculate V2
  }
}
