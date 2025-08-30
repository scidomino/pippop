uniform sampler2D u_Texture;
precision mediump float;
uniform vec4 u_Color;
varying vec2 v_TexCoordinate;
void main() {
  gl_FragColor = texture2D(u_Texture, v_TexCoordinate).w * u_Color;
}