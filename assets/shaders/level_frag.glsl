#version 330 core

out vec4 outColor;
in vec4 fragPos;

in vec2 tc;
uniform sampler2D tex;

void main()
{
	outColor = texture(tex, tc);

	float a = outColor.a;

	if(fract(abs(fragPos.x)) == 0.0)
		outColor *= 0.9;
	else if(fract(abs(fragPos.y)) == 0.0)
		outColor *= 0.8;

	outColor.a = a;
}