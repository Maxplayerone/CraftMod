use crate::renderer::buffer::Buffer;
use crate::renderer::program::ShaderProgram;
use crate::renderer::shader::{Shader, ShaderError};
use crate::renderer::vertex_array::VertexArray;

pub struct Renderer {
    program: ShaderProgram,
    vbo: Buffer,
    vao: VertexArray,
}

impl Renderer {
    pub fn new() -> Result<Self, ShaderError> {
        unsafe {
            let vertex_shader = Shader::new("src/shaders/basic.vs", gl::VERTEX_SHADER)?;
            let fragment_shader = Shader::new("src/shaders/basic.frag", gl::FRAGMENT_SHADER)?;
            let program = ShaderProgram::new(&[vertex_shader, fragment_shader])?;

            let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);

            let vertex_array = VertexArray::new();

            Ok(Self {
                program,
                vbo: vertex_buffer,
                vao: vertex_array,
            })
        }
    }

    pub fn upload_vbo_data(&self, data: &[f32]) {
        unsafe{
        self.vbo.set_data(data, gl::STATIC_DRAW);
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
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            self.program.bind();
            self.vao.bind();
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}
