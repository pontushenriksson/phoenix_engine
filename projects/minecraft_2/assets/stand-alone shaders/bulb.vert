#version 460 core

layout (location = 0) in vec3 aPos;      // Vertex position
layout (location = 1) in vec3 aColor;    // Vertex color
layout (location = 2) in vec2 aTexCoord; // Texture coordinates

uniform mat4 uModel;
uniform mat4 uView;
uniform mat4 uProjection;

out vec3 Color;
out vec2 TexCoord;

void main() {
  Color = aColor;
  aTexCoord;
  gl_Position = uProjection * uView * uModel * vec4(aPos, 1.0f);
}
