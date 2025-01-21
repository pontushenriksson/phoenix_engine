#version 460 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec4 aColor;
layout (location = 3) in vec2 aTexCoord;

uniform mat4 uModel;
uniform mat4 uView;
uniform mat4 uProjection;

out vec3 Normal;
out vec4 Color;
out vec2 TexCoord;

void main() {
  gl_Position = uProjection * uView * uModel * vec4(aPos, 1.0f);
  Normal = aNormal;
  Color = aColor;
  TexCoord = aTexCoord;
}
