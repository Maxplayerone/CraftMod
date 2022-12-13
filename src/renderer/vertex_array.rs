use gl::types::*;

pub struct VertexArray {
    pub id: GLuint,
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, [self.id].as_ptr());
        }
    }
}

impl VertexArray {
    pub unsafe fn new() -> Self {
        let mut id: GLuint = 0;
        gl::GenVertexArrays(1, &mut id);
        Self { id }
    }

    pub unsafe fn set_attribute(
        &self,
        attrib_pos: u32,
        num_of_components: i32,
        vertex_size: usize,
        offset: usize,
    ) {
        self.bind();
        gl::VertexAttribPointer(
            attrib_pos,
            num_of_components,
            gl::FLOAT,
            gl::FALSE,
            (vertex_size * std::mem::size_of::<GLfloat>()) as GLint,
            std::mem::transmute(offset * std::mem::size_of::<GLfloat>()),
        );
        gl::EnableVertexAttribArray(attrib_pos);
    }

    pub unsafe fn bind(&self) {
        gl::BindVertexArray(self.id);
    }
}
