#version 460 core

layout (location = 0) in vec3 aPos;

layout (location = 1) in vec2 aTex;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

// layout (location = 2) in vec3 aNormal;

// layout (location = 3) in vec4 aColor;

out vec2 tex;

// out vec3 normal;

// out vec4 color;

void main()
{
  tex = aTex;
  // normal = aNormal;
  // color = aColor;
  gl_Position = model * view * projection * vec4(aPos, 1.0f);
}
