#version 330 core

out vec4 outColor;
in vec4 fragPos;

uniform vec4 uColor;

void main()
{
	outColor = vec4(uColor.rgb, 1.0);

	if(abs(fragPos.y) == 1.0)
		outColor *= 0.9;
	else if(abs(fragPos.x) == 1.0)
		outColor *= 0.8;
	
	outColor.a = uColor.a;
}
