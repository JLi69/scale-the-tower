#version 330 core

out vec4 outColor;
in vec4 fragPos;

in vec2 tc;
uniform float uTexScale;
uniform vec2 uTexOffset;
uniform sampler2D tex;
uniform vec4 uColor;

void main()
{
	vec2 scaledTc = vec2(tc.x, 1.0 - tc.y) * uTexScale;
	outColor = texture(tex, scaledTc + vec2(uTexOffset.x, 1.0 - uTexOffset.y));

	outColor.r *= uColor.r;
	outColor.g *= uColor.g;
	outColor.b *= uColor.b;
	outColor.a *= uColor.a;
}
