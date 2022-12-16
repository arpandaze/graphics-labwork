mod opengl;

use opengl::*;

struct DDALine {
    points: Vec<f32>,
    x1: [f32; 2],
    x2: [f32; 2],
}

impl DDALine {
    fn generate_dda_line(&mut self) {
        let dx = self.x1[0] - self.x2[0];
        let dy = self.x1[1] - self.x2[1];

        let steps: u32;
        if dx.abs() > dy.abs() {
            steps = dx.abs() as u32;
        } else {
            steps = dy.abs() as u32;
        }

        let x_increment = dx / steps as f32;
        let y_increment = dy / steps as f32;

        let mut x = self.x1[0];
        let mut y = self.x1[1];

        for _ in 0..steps {
            x += x_increment;
            y += y_increment;
            self.points.push(x.round());
            self.points.push(y.round());
            // Z, R, G, B
            self.points.append(&mut [1.0,1.0,1.0,1.0].to_vec());
        }
    }
}

unsafe fn drawer(renderer: &mut opengl::Renderer) -> () {
    let vertex_shader =
        opengl::create_shader(&renderer.gl, gl::VERTEX_SHADER, VERTEX_SHADER_SOURCE);
    let fragment_shader = create_shader(&renderer.gl, gl::FRAGMENT_SHADER, FRAGMENT_SHADER_SOURCE);

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

    #[rustfmt::skip]
    let vertex_data: Vec<f32> = vec![
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
    ];

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
        gl::POINTS,
        vertex_indices.len() as i32,
        gl::UNSIGNED_INT,
        0 as *const _,
    );
}

const VERTEX_SHADER_SOURCE: &[u8] = b"
#version 100
precision mediump float;
attribute vec3 position;
attribute vec3 color;
varying vec3 v_color;
void main() {
    gl_Position = vec4(position, 1.0);
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

pub fn main() {
    opengl::init(Some(drawer));
}
