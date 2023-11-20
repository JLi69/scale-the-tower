#version 330 core

uniform vec2 uScreenDimensions;
uniform sampler2D tex;
in vec2 tc;
out vec4 outColor;

#define SIZE 64.0
#define TEXTURE_SCALE 8.0
#define OFFSET_X 1.0 / 8.0

void main()
{
	outColor = texture(
		tex, 
		vec2(
			fract(tc.x * uScreenDimensions.x / SIZE) / TEXTURE_SCALE + OFFSET_X,
			fract(tc.y * uScreenDimensions.y / SIZE) / TEXTURE_SCALE
		)
	);
}
