// #version 330 core
#version 460 core

// Position coordinates
layout (location = 0) in vec3 aPos;

// Colors
layout (location = 1) in vec3 aColor;

// Texture coordinates
layout (location = 2) in vec2 aTex;

// Output the color to the fragment shader
out vec3 color;

out vec2 texCoord;

void main()
{
  gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0f);
  vertexColor = aColor;
  texCoord = aTex;
}