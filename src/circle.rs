use crate::opengl;
use crate::opengl::*;
use ndarray::prelude::*;

pub struct Circle {
    points: Vec<[f32; 2]>,
    centre: [i32; 2],
    radius: i32,
}

impl Circle {
    pub fn new(centre: [i32; 2], radius: i32) -> Self {
        let mut circle = Self {
            points: vec![],
            centre,
            radius,
        };

        circle.generate_midpoint_circle();

        return circle;
    }

    fn generate_midpoint_circle(&mut self) {
        let mut x = 0;
        let mut y = self.radius;
        let mut d = 1 - self.radius;

        self.push_symmetric_points(x, y);

        while y > x {
            if d < 0 {
                d += 2 * x + 1;
            } else {
                d += 2 * (x - y) + 1;
                y -= 1;
            }
            x += 1;
            self.push_symmetric_points(x, y);
        }
    }

    fn push_symmetric_points(&mut self, x: i32, y: i32) {
        self.points
            .push([(self.centre[0] + x) as f32, (self.centre[1] + y) as f32]);
        self.points
            .push([(self.centre[0] - x) as f32, (self.centre[1] + y) as f32]);
        self.points
            .push([(self.centre[0] + x) as f32, (self.centre[1] - y) as f32]);
        self.points
            .push([(self.centre[0] - x) as f32, (self.centre[1] - y) as f32]);
        self.points
            .push([(self.centre[0] + y) as f32, (self.centre[1] + x) as f32]);
        self.points
            .push([(self.centre[0] - y) as f32, (self.centre[1] + x) as f32]);
        self.points
            .push([(self.centre[0] + y) as f32, (self.centre[1] - x) as f32]);
        self.points
            .push([(self.centre[0] - y) as f32, (self.centre[1] - x) as f32]);
    }

    pub fn get_normalized_coordinate(&self) -> Vec<f32> {
        return self
            .points
            .iter()
            .flat_map(|item| {
                [
                    (item[0] as f32 / 800.0) - 1.0,
                    (item[1] as f32 / 600.0) - 1.0,
                    0.0, // Z
                    1.0, // R
                    1.0, // G
                    1.0, // B
                ]
            })
            .collect();
    }

    pub fn get_2d_normalized_coordinate(&self) -> Vec<f32> {
        return self
            .points
            .iter()
            .flat_map(|item| {
                [
                    (item[0] as f32 / 800.0) - 1.0,
                    (item[1] as f32 / 600.0) - 1.0,
                    1.0, // R
                    1.0, // G
                    1.0, // B
                ]
            })
            .collect();
    }

    pub fn transform(&mut self, matrix: [[f32; 3]; 3]) {
        // let transformation_matrix = arr2(&matrix);
        //
        // let points = Array2::from(
        //     self.points
        //         .iter()
        //         .map(|x| [x[0] as f32, x[1] as f32, 1.0])
        //         .collect::<Vec<[f32; 3]>>(),
        // );
        // println!("{:?}",points.shape());
        // println!("{:?}",transformation_matrix.shape());
        //
        // let transformed_points = transformation_matrix
        //     .dot(&points)
        //     .iter()
        //     .map(|x| x.to_owned())
        //     .collect::<Vec<f32>>();
        //
        // println!("{:?}",self.points[93]);
        // println!("{:?}",transformed_points[93]);
        //
        // let mut new_points = Vec::new();
        // for i in 0..self.points.len() / 3 {
        //     new_points.push([transformed_points[i], transformed_points[i + 1]])
        // }
        //
        //
        // self.points = new_points;
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

        let circle = Circle::new([800, 600], 150);

        let vertex_data = circle.get_2d_normalized_coordinate();

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
            2,
            gl::FLOAT,
            0,
            5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
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
            5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
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
}

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

void main() {
    gl_Position = vec4(translation_mat * vec3(position, 1.0), 1.0);
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

#[test]
fn circle_test() {
    let mut circle = Circle::new([500, 500], 150);

    #[rustfmt::skip]
    let mat = [
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
    ];

    circle.transform(mat);
}
