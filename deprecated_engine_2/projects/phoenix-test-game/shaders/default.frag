// #version 330 core
#version 460 core

out vec4 FragColor;

in vec4 color;
in vec2 texCoord;
in vec3 Normal;
in vec3 currPos;

// Gets the Texture Unit from the main function
uniform sampler2D tex0;

uniform vec4 lightColor;
uniform vec3 lightPos;
uniform vec3 camPos;

void main() {
  float ambient = 0.20f;

  vec3 normal = normalize(Normal);
  vec3 lightDirection = normalize(lightPos - currPos);
  float diffuse = max(dot(normal, lightDirection), 0.0f);

  float specularLight = 0.50f;
  vec3 viewDirection = normalize(camPos - currPos);
  vec3 reflectionDirection = reflect(-lightDirection, normal);
  float specularAmount = pow(max(dot(viewDirection, reflectionDirection), 0.0f), 8); // Where 8 is the power
  float specular = specularAmount * specularLight;

  FragColor = texture(tex0, texCoord) * lightColor * (diffuse + ambient + specular);
}
