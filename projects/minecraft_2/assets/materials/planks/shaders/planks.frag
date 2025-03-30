#version 450 core

in vec3 Normal;
in vec4 Color;
in vec2 TexCoord;

uniform sampler2D Texture0;
uniform sampler2D Texture1;

out vec4 FragColor;

void main() {
  vec4 diffuse = texture(Texture0, TexCoord);
  vec4 specular = texture(Texture1, TexCoord);

  FragColor = diffuse;
}
