use crate::renderer::buffer::Buffer;
use crate::renderer::program::ShaderProgram;
use crate::renderer::shader::{Shader, ShaderError};
use crate::renderer::vertex_array::VertexArray;
use crate::renderer::texture::Texture;

extern crate cgmath;
use cgmath::{Matrix4, vec3, Deg, Rad, perspective};
use cgmath::prelude::*;

use std::path::Path;
use std::ffi::CStr;

//convert literals to c strings without any runtime overhead 
macro_rules! c_str {
    ($literal:expr) => {
        CStr::from_bytes_with_nul_unchecked(concat!($literal, "\0").as_bytes())
    }
}

pub struct Renderer {
    program: ShaderProgram,
    vbo: Buffer,
    ibo: Buffer,
    vao: VertexArray,
    tex0: Texture,
    tex1: Texture,
}

impl Renderer {
    pub fn new() -> Result<Self, ShaderError> {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);

            let vertex_shader = Shader::new("src/shaders/basic.vs", gl::VERTEX_SHADER)?;
            let fragment_shader = Shader::new("src/shaders/basic.frag", gl::FRAGMENT_SHADER)?;
            let program = ShaderProgram::new(&[vertex_shader, fragment_shader])?;

            let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
            let element_buffer = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);

            let vertex_array = VertexArray::new();

            let tex0 = Texture::new(Path::new("src/resources/container.jpg"));
            let tex1 = Texture::new(Path::new("src/resources/awesomeface.png"));
            
            program.set_int(c_str!("tex0"), 0);
            program.set_int(c_str!("tex1"), 1);

            Ok(Self {
                program,
                vbo: vertex_buffer,
                ibo: element_buffer,
                vao: vertex_array,
                tex0: tex0,
                tex1: tex1,
            })
        }
    }

    pub fn upload_vbo_data(&self, data: &[f32]) {
        unsafe {
            self.vbo.set_data(data, gl::STATIC_DRAW);
        }
    }

    pub fn upload_ibo_data(&self, data: &[u32]) {
        unsafe {
            self.vao.bind();
            self.ibo.set_data(data, gl::STATIC_DRAW);
        }
    }

    pub fn set_vao_attrib(
        &self,
        loc: u32,
        num_of_components: i32,
        vertex_size: usize,
        offset: usize,
    ) {
        unsafe {
            self.vao.bind();
            self.vbo.bind();
            self.vao
                .set_attribute(loc, num_of_components, vertex_size, offset);
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
           
            gl::ActiveTexture(gl::TEXTURE0);
            self.tex0.bind();
            gl::ActiveTexture(gl::TEXTURE1);
            self.tex1.bind();
            self.program.bind();

            let scr_width = 800;
            let scr_height = 600;
            let model: Matrix4<f32> = Matrix4::from_axis_angle(vec3(0.5, 1.0, 0.0).normalize(),
                                                                Rad(32.0 as f32));
            let view: Matrix4<f32> = Matrix4::from_translation(vec3(0.0, 0.0, -3.0));
            let projection: Matrix4<f32> = perspective(Deg(45.0), scr_width as f32 / scr_height as f32, 0.1, 100.0);

            //uniform locations
            let model_loc = gl::GetUniformLocation(self.program.id, c_str!("model").as_ptr());
            let view_loc = gl::GetUniformLocation(self.program.id, c_str!("view").as_ptr());
            let proj_loc = gl::GetUniformLocation(self.program.id, c_str!("projection").as_ptr());
            gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, model.as_ptr());
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, &view[0][0]);
            gl::UniformMatrix4fv(proj_loc, 1, gl::FALSE, projection.as_ptr());

            self.vao.bind();
            //gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
    }

}
