#version 450 core

in vec3 Color;
in vec2 TexCoord;

uniform sampler2D Texture1;

out vec4 FragColor;

void main() {
  FragColor = texture(Texture1, TexCoord) * vec4(0.25f, 0.25f, 0.5f, 1.0f);
}
