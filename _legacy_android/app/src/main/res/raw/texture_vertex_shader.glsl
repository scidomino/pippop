uniform mat2 uMVPMatrix;
attribute vec2 vPosition;
attribute vec2 vTexCoordinate;
varying vec2 iTexCoordinate;
void main() {
  iTexCoordinate = vTexCoordinate;
  gl_Position = vec4(vPosition * uMVPMatrix, 0.0, 1.0);
}