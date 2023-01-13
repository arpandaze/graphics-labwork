use crate::opengl;
use crate::opengl::*;

const VERTEX_SHADER_SOURCE: &[u8] = b"
#version 100
precision mediump float;
attribute vec2 position;
attribute vec3 color;
varying vec3 v_color;

const float PI = 3.141592654;

const float ANGLE = PI/2.0;

mat3 rotation_mat = mat3(
    cos(ANGLE), sin(ANGLE),   0.0,
    -sin(ANGLE), cos(ANGLE),   0.0,
    0.0,      0.0,   1.0
);

mat3 original = mat3(
    1.0, 0.0, 0.0,
    0.0, 1.0, 0.0,
    0.0, 0.0, 1.0
);

mat3 translation_mat = mat3(
    1.0, 0.0, 0.0,
    0.0, 1.0, 0.0,
    -0.5, -0.2, 1.0
);

mat3 scaling_mat = mat3(
    2.0, 0.0, 0.0,
    0.0, 2.0, 0.0,
    0.0, 0.0, 1.0
);

// Reflection in XY
mat3 reflection_mat_1 = mat3(
    0.0, 1.0, 0.0,
    1.0, 0.0, 0.0,
    0.0, 0.0, 1.0
);

// Reflection in X = -Y
mat3 reflection_mat_2 = mat3(
    0.0, -1.0, 0.0,
    -1.0, 0.0, 0.0,
    0.0, 0.0, 1.0
);

// Reflection in X
mat3 reflection_mat_3 = mat3(
    1.0,  0.0, 0.0,
    0.0, -1.0, 0.0,
    0.0,  0.0, 1.0
);

// Reflection in Y
mat3 reflection_mat_4 = mat3(
    -1.0, 0.0, 0.0,
    0.0,  1.0, 0.0,
    0.0,  0.0, 1.0
);

// Reflection about origin
mat3 reflection_mat_5 = mat3(
    -1.0, 0.0, 0.0,
    0.0,  -1.0, 0.0,
    0.0,  0.0, 1.0
);

// Y Shear
mat3 shear_y = mat3(
    1.0, 0.2, 0.0,
    0.0,  1.0, 0.0,
    0.0,  0.0, 1.0
);

// X Shear
mat3 shear_x = mat3(
    1.0, 0.0, 0.0,
    0.2,  1.0, 0.0,
    0.0,  0.0, 1.0
);

void main() {
    gl_Position = vec4(shear_y * vec3(position, 1.0), 1.0);
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

pub struct Flag {
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
}

impl Flag {
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

        let line = Self::default();

        let vertex_data = line.vertices;
        println!("{:?}", vertex_data);

        let vertex_indices: Vec<u32> = (0..vertex_data.len() as u32).collect();

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

impl Default for Flag {
    fn default() -> Self {
        Self {
            #[rustfmt::skip]
            vertices: vec![
                // Blue Triangle 1
                -0.3, 0.45, 1.0,0.0, 0.2196, 0.572549,
                -0.3, -0.013, 1.0,0.0, 0.2196, 0.572549,
                0.43125, -0.013, 1.0,0.0, 0.2196, 0.572549,

                // Blue Triangle 2
                -0.3, 0.25625, 1.0,0.0, 0.2196, 0.572549,
                -0.3, -0.45, 1.0,0.0, 0.2196, 0.572549,
                0.41, -0.45, 1.0,0.0, 0.2196, 0.572549,


                // Red Triangle 1
                -0.27296875, 0.4015625, 1.0, 0.862745, 0.078431, 0.235294,
                -0.27296875, 0.0140625, 1.0, 0.862745, 0.078431, 0.235294,
                0.344375, 0.0140625, 1.0, 0.862745, 0.078431, 0.235294,

                // Red Triangle 2
                -0.274, 0.19890625, 1.0, 0.862745, 0.078431, 0.235294,
                -0.274, -0.42234375, 1.0, 0.862745, 0.078431, 0.235294,
                0.34421875, -0.42234375, 1.0, 0.862745, 0.078431, 0.235294,
            ],
            indices: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
        }
    }
}
