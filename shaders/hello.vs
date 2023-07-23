#version 460 core
layout (location = 0) in vec3 i_pos;
layout (location = 1) in vec3 i_col;
layout (location = 2) in vec2 i_uv;
layout (location = 3) in vec3 i_normal;
out vec3 o_pos;
out vec3 o_col;
out vec2 o_uv;
out vec3 o_normal;

uniform mat4 iRot;
uniform mat4 view;
uniform mat4 cam;

void main()
{
    o_pos =  i_pos;
    o_col =  i_col;
    o_uv = i_uv;
    o_normal = i_normal;
    gl_Position = cam * view * vec4(i_pos, 1.0);
    //gl_Position = vec4(i_pos, 1.0);
}
