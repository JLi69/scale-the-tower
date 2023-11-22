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
	vec2 scaledTc = vec2(max(min(tc.x, 0.99), 0.01), max(min(1.0 - tc.y, 0.99), 0.01)) * uTexScale;
	if(uFlipped)
		scaledTc.x = uTexScale - scaledTc.x;

	outColor = texture(tex, scaledTc + uTexOffset);
	
	if(outColor.a < 0.01)
		discard;
}
