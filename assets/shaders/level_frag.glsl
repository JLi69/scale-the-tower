#version 330 core

out vec4 outColor;
in vec4 fragPos;

in vec2 tc;
uniform sampler2D tex;
uniform float uAnimationTimer;
in float animation;

void main()
{
	vec4 col = texture(tex, tc + vec2(0.0, animation * floor(uAnimationTimer)));

	if(col.a < 0.01)
		discard;

	outColor = col;

	float a = outColor.a;

	if(fract(abs(fragPos.x)) == 0.0)
		outColor *= 0.9;
	else if(fract(abs(fragPos.y)) == 0.0)
		outColor *= 0.8;

	outColor.a = a;
}
