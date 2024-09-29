#version 330

layout(location = 0) in vec4 position;
uniform vec2 offset;

void main()
{
    vec4 dp = vec4(offset.x, offset.y, 0.0f, 0.0f);
    gl_Position = position + dp;
}
