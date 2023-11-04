#version 330 core

out vec4 outColor;
in vec4 fragPos;

in vec2 tc;
uniform float uTexScale;
uniform vec2 uTexOffset;
uniform sampler2D tex;

void main()
{
	outColor = texture(tex, vec2(tc.x, 1.0 - tc.y) * uTexScale + uTexOffset);
}
