#version 460 core

layout (location = 0) in vec3 aPosition;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec4 aColor;
layout (location = 3) in vec2 aTexCoord;

out vec3 Normal;
out vec4 Color;
out vec2 TexCoord;

void main() {
  gl_Position = vec4(aPosition, 1.0f);
  Normal = aNormal;
  Color = aColor;
  TexCoord = aTexCoord;
}
