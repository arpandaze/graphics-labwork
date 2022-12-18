pub enum LineAlgorithm {
    DDA,
    Bresenham,
    Midpoint,
}

pub struct Line {
    points: Vec<[f32; 2]>,
    x1: [f32; 2],
    x2: [f32; 2],
}

impl Line {
    pub fn new(x1: [f32; 2], x2: [f32; 2], algo: LineAlgorithm) -> Self {
        let mut line = Self {
            x1,
            x2,
            points: vec![],
        };

        match algo {
            LineAlgorithm::DDA => line.generate_dda_line(),
            LineAlgorithm::Bresenham => line.generate_bresenham_line(),
            LineAlgorithm::Midpoint => line.generate_midpoint_line(),
        }

        return line;
    }

    fn generate_dda_line(&mut self) {
        let dx = self.x2[0] - self.x1[0];
        let dy = self.x2[1] - self.x1[1];

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

        self.points.push([x.round(), y.round()]);
        for _ in 0..steps {
            x += x_increment;
            y += y_increment;
            self.points.push([x.round(), y.round()]);
        }
    }

    fn generate_bresenham_line(&mut self) {
        let x0 = self.x1[0];
        let x1 = self.x2[0];
        let y0 = self.x1[1];
        let y1 = self.x2[1];

        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();

        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };

        let mut x = x0;
        let mut y = y0;

        if dx > dy {
            let mut err = dx / 2.0;

            while x != x1 {
                self.points.push([x, y]);

                err -= dy;
                if err < 0.0 {
                    y += sy as f32;
                    err += dx;
                }

                x += sx as f32;
            }
        } else {
            let mut err = dy / 2.0;

            while y != y1 {
                self.points.push([x, y]);

                err -= dx;
                if err < 0.0 {
                    x += sx as f32;
                    err += dy;
                }

                y += sy as f32;
            }
        }

        self.points.push([x, y]);
    }

    fn generate_midpoint_line(&mut self) {
        let x0 = self.x1[0];
        let x1 = self.x2[0];
        let y0 = self.x1[1];
        let y1 = self.x2[1];

        let mut x = x0;
        let mut y = y0;
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };

        if dy >= dx {
            let mut err = if dx > dy { dy } else { -dx } / 2.0;

            loop {
                self.points.push([x, y]);
                if x == x1 && y == y1 {
                    break;
                }
                let e2 = err;
                if e2 > -dx {
                    err -= dy;
                    x += sx as f32;
                }
                if e2 < dy {
                    err += dx;
                    y += sy as f32;
                }
            }
        } else {
            let mut err = if dx > dy { dx } else { -dy } / 2.0;

            loop {
                self.points.push([x, y]);
                if x == x1 && y == y1 {
                    break;
                }
                let e2 = err;
                if e2 > -dx {
                    err -= dy;
                    x += sx as f32;
                }
                if e2 < dy {
                    err += dx;
                    y += sy as f32;
                }
            }
        }
    }

    pub fn get_normalized_coordinate(&self) -> Vec<f32> {
        return self
            .points
            .iter()
            .flat_map(|item| {
                [
                    (item[0] / 800.0) - 1.0,
                    (item[1] / 600.0) - 1.0,
                    0.0, // Z
                    1.0, // R
                    1.0, // G
                    1.0, // B
                ]
            })
            .collect();
    }
}
