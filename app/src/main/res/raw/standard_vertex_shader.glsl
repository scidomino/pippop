uniform mat2 uMVPMatrix;
attribute vec2 vPosition;
void main() {
  gl_Position = vec4(vPosition * uMVPMatrix, 0.0, 1.0);
}