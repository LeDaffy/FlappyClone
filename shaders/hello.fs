#version 460 core
out vec4 FragColor;

in vec3 o_pos;
in vec3 o_col;
in vec2 o_uv;
in vec3 o_normal;

uniform sampler2D tex1;

uniform float iTime;
void main()
{
   FragColor = vec4(o_uv, 0.0, 1.0);
   // FragColor = texture(tex1, o_uv);
}
