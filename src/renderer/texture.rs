use gl::{self, types::*};
use png;
use std::fs::File;

pub enum TextureFormat {
    DepthComponent,
    DepthStencil,
    Red,
    Rg,
    Rgb,
    Rgba,
}
pub enum BitDepth {
    Byte(u8),
    Short(u16),
    Float(f32),
}

pub struct Texture {
    pub id: GLuint,
    pub format: TextureFormat,
    pub bytes: Vec<u8>,
}

impl Texture {
    pub fn from_path(path: &std::path::Path) -> Texture {
        let decoder = png::Decoder::new(File::open(path).unwrap());
        let mut reader = decoder.read_info().unwrap();
        // Allocate the output buffer.
        let mut buf = vec![0; reader.output_buffer_size()];
        println!(
            "BD: {:?}\nCT: {:?}",
            reader.info().bit_depth,
            reader.info().color_type
        );
        // Read the next frame. An APNG might contain multiple frames.
        let info = reader.next_frame(&mut buf).unwrap();
        // Grab the bytes of the image.
        let bytes = &buf[..info.buffer_size()];
        // Inspect more details of the last read frame.
        let in_animation = reader.info().frame_control.is_some();

        let mut id: GLuint = 0;

        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // set texture wrapping to GL_REPEAT (default wrapping method)
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                reader.info().width as i32,
                reader.info().height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                bytes.as_ptr() as *const std::ffi::c_void,
                );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        Self {
            id: id,
            format: TextureFormat::Rgba,
            bytes: Vec::new(),
        }
    }
}
