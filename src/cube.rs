use crate::opengl;
use crate::opengl::*;

pub struct Cube {
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
}

impl Cube {
    fn new(length: f32) -> Self {
        let vertices = vec![
            -length as f32 / 2.0,
            -length as f32 / 2.0,
            length as f32 / 2.0,
            1.0,
            1.0,
            1.0,
            length as f32 / 2.0,
            -length as f32 / 2.0,
            length as f32 / 2.0,
            1.0,
            1.0,
            1.0,
            length as f32 / 2.0,
            length as f32 / 2.0,
            length as f32 / 2.0,
            1.0,
            1.0,
            1.0,
            -length as f32 / 2.0,
            length as f32 / 2.0,
            length as f32 / 2.0,
            1.0,
            1.0,
            1.0,
            -length as f32 / 2.0,
            -length as f32 / 2.0,
            -length as f32 / 2.0,
            1.0,
            1.0,
            1.0,
            length as f32 / 2.0,
            -length as f32 / 2.0,
            -length as f32 / 2.0,
            1.0,
            1.0,
            1.0,
            length as f32 / 2.0,
            length as f32 / 2.0,
            -length as f32 / 2.0,
            1.0,
            1.0,
            1.0,
            -length as f32 / 2.0,
            length as f32 / 2.0,
            -length as f32 / 2.0,
            1.0,
            1.0,
            1.0,
        ];

        let indices = vec![
            0, 1, 2, 2, 3, 0, // front
            1, 5, 6, 6, 2, 1, // right
            5, 4, 7, 7, 6, 5, // back
            4, 0, 3, 3, 7, 4, // left
            3, 2, 6, 6, 7, 3, // top
            4, 5, 1, 1, 0, 4, // bottom
        ];

        return Self { vertices, indices };
    }

    pub unsafe fn drawer(renderer: &mut crate::opengl::Renderer) -> () {
        let vertex_shader =
            opengl::create_shader(&renderer.gl, gl::VERTEX_SHADER, VERTEX_SHADER_SOURCE);
        let fragment_shader =
            create_shader(&renderer.gl, gl::FRAGMENT_SHADER, FRAGMENT_SHADER_SOURCE);

        renderer.program = Some(renderer.gl.CreateProgram());

        renderer
            .gl
            .AttachShader(renderer.program.unwrap(), vertex_shader);

        renderer
            .gl
            .AttachShader(renderer.program.unwrap(), fragment_shader);

        renderer.gl.LinkProgram(renderer.program.unwrap());

        renderer.gl.UseProgram(renderer.program.unwrap());

        renderer.gl.GenVertexArrays(1, &mut renderer.vao);
        renderer.gl.BindVertexArray(renderer.vao);

        renderer.gl.GenBuffers(1, &mut renderer.vbo);
        renderer.gl.BindBuffer(gl::ARRAY_BUFFER, renderer.vbo);

        let cube = Cube::new(0.5);

        let vertex_data = cube.vertices;

        let vertex_indices: Vec<u32> = cube.indices;

        let mut indices: gl::types::GLuint = std::mem::zeroed();

        renderer.gl.GenBuffers(1, &mut indices);
        renderer.gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, indices);
        renderer.gl.BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (vertex_indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
            vertex_indices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        renderer.gl.BufferData(
            gl::ARRAY_BUFFER,
            (vertex_data.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertex_data.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        // POSITION Attribute
        let pos_attrib = renderer.gl.GetAttribLocation(
            renderer.program.unwrap(),
            b"position\0".as_ptr() as *const _,
        );

        renderer.gl.VertexAttribPointer(
            pos_attrib as gl::types::GLuint,
            3,
            gl::FLOAT,
            0,
            6 * std::mem::size_of::<f32>() as gl::types::GLsizei,
            std::ptr::null(),
        );

        // COLOR Attribute
        let color_attrib = renderer
            .gl
            .GetAttribLocation(renderer.program.unwrap(), b"color\0".as_ptr() as *const _);

        renderer.gl.VertexAttribPointer(
            color_attrib as gl::types::GLuint,
            3,
            gl::FLOAT,
            0,
            6 * std::mem::size_of::<f32>() as gl::types::GLsizei,
            (3 * std::mem::size_of::<f32>()) as *const _,
        );

        renderer
            .gl
            .EnableVertexAttribArray(pos_attrib as gl::types::GLuint);

        renderer
            .gl
            .EnableVertexAttribArray(color_attrib as gl::types::GLuint);

        renderer.gl.ClearColor(0.1, 0.1, 0.1, 0.9);

        renderer.gl.Clear(gl::COLOR_BUFFER_BIT);

        renderer.gl.DrawElements(
            gl::TRIANGLES,
            vertex_indices.len() as i32,
            gl::UNSIGNED_INT,
            0 as *const _,
        );
    }
}

const VERTEX_SHADER_SOURCE: &[u8] = b"
#version 100
precision mediump float;
attribute vec3 position;
attribute vec3 color;
varying vec3 v_color;

const float PI = 3.141592654;

float c = cos(-PI/3.0);
float s = sin(-PI/3.0);

mat4 x_rotate = mat4(
    1.0, 0.0, 0.0, 0.0,
    0.0, c, -s, 0.0,
    0.0, s, c, 0.0,
    0.0, 0.0, 0.0, 1.0
); 

mat4 y_rotate = mat4(
    c, 0.0, s, 0.0,
    0.0, 1.0, 0.0, 0.0,
    -s, 0.0, c, 0.0,
    0.0, 0.0, 0.0, 1.0
); 

mat4 z_rotate = mat4(
    c, -s, 0.0, 0.0,
    s, c, 0.0, 0.0,
    0.0, 0.0, 1.0, 0.0,
    0.0, 0.0, 0.0, 1.0
);

mat4 translation = mat4(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 1.0, 0.0,
    0.5, 0.25, 0.0, 1.0
);

mat4 scaling = mat4(
    2.0, 0, 0, 0,
    0, 0.5, 0, 0,
    0, 0, 1, 0,
    0, 0, 0, 1
);

mat4 uni_scaling = mat4(
    0.5, 0, 0, 0,
    0, 0.5, 0, 0,
    0, 0, 0.5, 0,
    0, 0, 0, 1
);

void main() {
    gl_Position = uni_scaling * x_rotate * y_rotate * z_rotate * vec4(position, 1.0);
    v_color = color;
}
\0";

const FRAGMENT_SHADER_SOURCE: &[u8] = b"
#version 100
precision mediump float;
varying vec3 v_color;
void main() {
    gl_FragColor = vec4(v_color, 1.0);
}

\0";
