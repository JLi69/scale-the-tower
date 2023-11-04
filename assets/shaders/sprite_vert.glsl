#version 330 core

layout(location = 0) in vec4 pos;
layout(location = 1) in vec2 textureCoordinate;

uniform mat4 uPerspective;
uniform mat4 uTransform;
uniform mat4 uView;

out vec4 fragPos;
out vec2 tc;

void main()
{
	gl_Position = uPerspective * uView * uTransform * pos;
	fragPos = pos;
	tc = textureCoordinate;
}
