#version 460 core

in vec3 Color;
in vec2 TexCoord;

uniform sampler2D Texture0;
uniform sampler2D Texture1;

out vec4 FragColor;

void main() {
  FragColor = texture(Texture1, TexCoord);
}
