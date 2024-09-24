#version 330

out vec4 outputColor;
void main()
{
    float lerpValue = gl_FragCoord.y / 500.0f;
    outputColor = mix(vec4(1.0f, 1.0f, 1.0f, 1.0f),
        vec4(0.1f, 0.1, 0.1f, 1.0f), lerpValue);
}
