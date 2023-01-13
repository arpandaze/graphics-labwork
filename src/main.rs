mod circle;
mod ellipse;
mod flag;
mod line;
mod opengl;
mod cube;

use circle::Circle;
use ellipse::Ellipse;
use line::{Line, LineAlgorithm};
use opengl::*;

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

    let tm = [
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
    ];

    let mut line = circle::Circle::new([500, 500], 150);
    line.transform(tm);

    let vertex_data = line.get_normalized_coordinate();

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
    // let circle = Circle::new([350, 400], 150);
    unsafe {
        opengl::init(Some(cube::Cube::drawer));
    }
}
