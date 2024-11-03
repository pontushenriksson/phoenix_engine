#version 330 core
out vec4 FragColor;
in vec4 vertexColor;

// uniform vec4 aTextCoord; // we set this variable in the OpenGL code.

/*

If you declare a uniform that isn't used anywhere in your GLSL code
the compiler will silently remove the variable from the compiled version
which is the cause for several frustrating errors; keep this in mind! 

*/

void main()
{
  FragColor = vertexColor; // vec3(R, G, B, 1.0)
}
