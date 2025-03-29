// #version 330 core
#version 460 core

// Position coordinates
layout (location = 0) in vec3 aPos;

// Colors
layout (location = 1) in vec4 aColor;

// Texture coordinates
layout (location = 2) in vec2 aTex;

// Normals
layout (location = 3) in vec3 aNormal;

// Output the color to the fragment shader
out vec4 color;

out vec2 texCoord;

out vec3 Normal;
out vec3 currPos;

uniform mat4 camMatrix;
uniform mat4 model;

void main()
{
  currPos = vec3(model * vec4(aPos, 1.0f));
  gl_Position = camMatrix * vec4(currPos, 1.0f);
  color = aColor;
  texCoord = aTex;
  Normal = aNormal;
}