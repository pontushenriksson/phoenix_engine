#version 450 core

in vec3 Color;
in vec2 TexCoord;

uniform sampler2D Texture0;
uniform sampler2D Texture1;

out vec4 FragColor;

void main() {
  vec4 texture1 = texture(Texture1, TexCoord);
  FragColor = texture(Texture0, TexCoord) * vec4(0.25f, 0.25f, 0.5f, 1.0f);
}
