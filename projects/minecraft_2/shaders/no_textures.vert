#version 460 core

layout (location = 0) in vec3 aPos;

layout (location = 1) in vec3 aNormal;

layout (location = 2) in vec3 aColor;

out vec2 tex;

out vec3 normal;

out vec3 color;

void main()
{
  normal = aNormal;
  color = aColor;
  gl_Position = vec4(aPos, 1.0f);
}
