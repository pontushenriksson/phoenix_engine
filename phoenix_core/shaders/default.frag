#version 330 core // 460

out vec4 FragColor;

in vec4 vertexColor;
in vec2 texCoord;

/*

If you declare a uniform that isn't used anywhere in your GLSL code
the compiler will silently remove the variable from the compiled version
which is the cause for several frustrating errors; keep this in mind! 

*/

uniform sampler2D Texture;

void main()
{
  FragColor = texture(Texture, texCoord); // * vertexColor; // vec3(R, G, B, 1.0)
}
