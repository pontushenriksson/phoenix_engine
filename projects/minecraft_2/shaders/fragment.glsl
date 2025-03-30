#version 460 core

in vec2 tex;

// in vec3 normal;

// in vec4 color;

uniform sampler2D tex0;

out vec4 FragColor;

void main()
{ 
  FragColor = texture(tex0, tex);
}
