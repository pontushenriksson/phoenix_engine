#version 450 core

layout (location = 0) in vec3 aPos;      // Vertex position
layout (location = 1) in vec3 aColor;    // Vertex color
layout (location = 2) in vec2 aTexCoord; // Texture coordinates

uniform mat4 uModel;                     // Model transformation matrix
uniform mat4 uView;                      // View transformation matrix
uniform mat4 uProjection;                // Projection transformation matrix
uniform sampler2D Texture0;              // Heightmap texture
uniform float uHeightScale = 0.2f;       // Scale factor for height displacement

out vec3 Color;                          // Pass-through color
out vec2 TexCoord;                       // Pass-through texture coordinates

void main() {
  // float uHeightScale = 0.05f;
  // Sample the heightmap texture to get the height value (assuming grayscale heightmap)
  float height = texture(Texture0, aTexCoord).r; // Use the red channel for height

  // Displace the vertex position along the y-axis based on the heightmap and scale factor
  vec3 displacedPosition = aPos + vec3(0.0, height * uHeightScale, 0.0);

  // Pass the attributes to the fragment shader
  Color = aColor;
  TexCoord = aTexCoord;

  // Transform the displaced position to clip space
  gl_Position = uProjection * uView * uModel * vec4(displacedPosition, 1.0f);
}
