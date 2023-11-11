#version 330 core

layout(location = 0) in vec4 pos;
layout(location = 1) in vec2 textureCoordinate;

uniform float uScale; //In pixels
uniform vec2 uPosition; //In pixels
uniform vec2 uScreenDimensions;

out vec2 tc;

void main()
{
	mat2x2 scaleMatrix = mat2x2(
		1.0 / uScreenDimensions.x * 2.0, 0.0,
		0.0, 1.0 / uScreenDimensions.y * 2.0
	);
	gl_Position = 
		vec4(
			(pos.xy * uScale + uPosition) * scaleMatrix, 0.0, 1.0
		);
	tc = textureCoordinate;
}
