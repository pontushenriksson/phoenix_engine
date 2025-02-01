#version 460 core

in vec3 Color;
in vec2 TexCoord;

uniform sampler2D Texture0;

out vec4 FragColor;

void main() {
  FragColor = texture(Texture0, TexCoord) * vec4(0.5f, 0.5f, 1.0f, 0.5f);
}
