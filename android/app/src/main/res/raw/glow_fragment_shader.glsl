precision mediump float;
uniform vec4 uColor;
varying float iAlpha;
void main() {
  gl_FragColor = uColor;
  gl_FragColor.a *= iAlpha;
}