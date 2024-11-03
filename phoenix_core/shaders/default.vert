#version 330 core /* 460 */
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec4 aColor;
layout (location = 2) in vec2 aTexCoord;

out vec4 vertexColor; // output a color to the fragment shader
out vec2 TexCoord;    // output texture coordinate to fragment shader

void main()
{
  gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0f);
  vertexColor = aColor;
  TexCoord = aTexCoord;
}
