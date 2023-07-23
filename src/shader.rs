use colored::Colorize;
use gl::{self, types::*};
use std::ffi::CString;

pub struct Shader {
    pub id: GLuint,
}

impl Shader {
    pub fn enable(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn set_float(&self, name: &str, value: f32) {
        let name = CString::new(name).unwrap();
        unsafe {
        gl::Uniform1f(
            gl::GetUniformLocation(self.id, name.as_ptr()),
            value,
        );
        }
    }
    pub fn set_mat4(&self, name: &str, mat: &nalgebra::base::Matrix4<f32>) {
        let name = CString::new(name).unwrap();
        unsafe {
        gl::UniformMatrix4fv(gl::GetUniformLocation(self.id, name.as_ptr()), 1, gl::FALSE, mat.as_ptr());
        }
    }
    pub fn new(vs_path: &std::path::Path, fs_path: &std::path::Path) -> Shader {
        let vs_source = CString::new(std::fs::read_to_string(vs_path).unwrap()).unwrap();
        let vs = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };

        unsafe {
            gl::ShaderSource(
                vs,
                1,
                &(vs_source.as_ptr()) as *const *const GLchar,
                std::ptr::null(),
            );
            gl::CompileShader(vs);
        }
        Self::print_shader_compilation(vs, vs_path);

        let fs_source = CString::new(std::fs::read_to_string(fs_path).unwrap()).unwrap();
        let fs = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };

        unsafe {
            gl::ShaderSource(
                fs,
                1,
                &(fs_source.as_ptr()) as *const *const GLchar,
                std::ptr::null(),
            );
            gl::CompileShader(fs);
        }
        Self::print_shader_compilation(fs, fs_path);

        // shader Program
        let id = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(id, vs);
            gl::AttachShader(id, fs);
            gl::LinkProgram(id);
        }
        Self::print_shader_link(id);

        Shader { id: id }
    }

    fn print_shader_compilation(shader: u32, path: &std::path::Path) {
        let mut success = 0;
        unsafe {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        }
        if success == 0 {
            print!("{0} ", path.to_str().unwrap().bold());
            let info_log: Vec<u8> = vec![0; 1024];
            let info_log = unsafe { CString::from_vec_unchecked(info_log) };
            unsafe {
                gl::GetShaderInfoLog(
                    shader,
                    1024,
                    std::ptr::null_mut(),
                    info_log.as_ptr() as *mut i8,
                );
            }
            let info_log = info_log.into_string().unwrap();
            info_log.split_whitespace().for_each(|w| match w {
                "error:" => {
                    print!("{0}", "error: ".bold().red());
                }
                "warning:" => {
                    print!("{0}", "warning: ".bold().red());
                }
                _ => {
                    print!("{0} ", w.normal());
                }
            });
            println!("");
        }
    }
    fn print_shader_link(program: u32) {
        let mut success = 0;
        unsafe {
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        }
        if success == 0 {
            let info_log: Vec<u8> = vec![0; 1024];
            let info_log = unsafe { CString::from_vec_unchecked(info_log) };
            unsafe {
                gl::GetProgramInfoLog(
                    program,
                    1024,
                    std::ptr::null_mut(),
                    info_log.as_ptr() as *mut i8,
                );
            }
            let info_log = info_log.into_string().unwrap();
            info_log.split_whitespace().for_each(|w| match w {
                "error:" => {
                    print!("{0}", "error: ".bold().red());
                }
                "warning:" => {
                    print!("{0}", "warning: ".bold().red());
                }
                _ => {
                    print!("{0} ", w.normal());
                }
            });
            println!("");
        }
    }
}
