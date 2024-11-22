// #version 330 core
#version 460 core

out vec4 FragColor;

in vec4 color;

in vec2 texCoord;

uniform sampler2D tex0;

void main() {
  FragColor = texture(tex0, texCoord); // vec4(color, 1.0f);
}
