#version 330

uniform vec2 offset;
const vec4 color1 = vec4(1.0f, 0.0f, 1.0f, 1.0f);
const vec4 color2 = vec4(0.0f, 1.0f, 0.0f, 1.0f);

out vec4 outputColor;

void main()
{
    // use x position to determine interpolation
    float alpha = offset.x + 0.5f;
    outputColor = mix(color1, color2, alpha);
}
