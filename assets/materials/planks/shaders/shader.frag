#version 460 core

in vec2 TexCoord;

out vec4 FragColor;

void main() {
  FragColor = texture(Texture0, TexCoord); // texture(Texture1, TexCoord) * specular...
}
