#version 330 core

out vec4 outColor;
in vec4 fragPos;

in vec2 tc;
uniform float uTexScale;
uniform vec2 uTexOffset;
uniform sampler2D tex;
uniform bool uFlipped;

void main()
{
	vec2 scaledTc = vec2(tc.x, 1.0 - tc.y) * uTexScale;
	if(uFlipped)
		scaledTc.x = uTexScale - scaledTc.x;

	outColor = texture(tex, scaledTc + uTexOffset);
}
