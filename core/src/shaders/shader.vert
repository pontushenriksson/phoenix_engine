#version 460 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;

out vec3 color;

uniform float scale; // Never declare if not used!

void main() {
  gl_Position = vec4(aPos.x + aPos.x * scale, aPos.y + aPos.x * scale, aPos.z + aPos.x * scale, 1.0);
  color = aColor;
}
