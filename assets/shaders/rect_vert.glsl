#version 330 core

layout(location = 0) in vec4 pos;
layout(location = 1) in vec2 textureCoordinate;

out vec2 tc;

void main()
{
	gl_Position = pos;
	tc = textureCoordinate;
}
