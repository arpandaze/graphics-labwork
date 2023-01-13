pub struct Ellipse {
    points: Vec<[i32; 2]>,
    centre: [i32; 2],
    a: i32,
    b: i32,
}

impl Ellipse {
    pub fn new(centre: [i32; 2], a: i32, b: i32) -> Self {
        let mut ellipse = Self {
            points: vec![],
            centre,
            a,
            b,
        };

        ellipse.generate_midpoint_ellipse();

        return ellipse;
    }

    fn generate_midpoint_ellipse(&mut self) {
        let mut x = 0;
        let mut y = self.b;

        let a2 = (self.a * self.a) as f32;
        let b2 = (self.b * self.b) as f32;
        let mut d = (b2 - a2 * self.b as f32 + (a2 / 4.0)) as f32;

        self.push_symmetric_points(x, y);

        // Region 1
        while (a2 as f32 * (y as f32 - 0.5)) > (b2 as f32 * (x + 1) as f32) {
            if d < 0.0 {
                // Select E
                d += b2 * (2 * x + 3) as f32;
            } else {
                // Select SE
                d += b2 * (2 * x + 3) as f32 + a2 * (-2 * y + 2) as f32;
                y -= 1;
            }
            x += 1;

            self.push_symmetric_points(x, y);
        }

        // Region 2
        d = b2 as f32 * (x as f32 + 0.5) * (x as f32 + 0.5)
            + a2 as f32 * (y - 1) as f32 * (y - 1) as f32
            - a2 as f32 * b2 as f32;
        while y > 0 {
            if d < 0.0 {
                // Select SE
                d += b2 * (2 * x + 2) as f32 + a2 * (-2 * y + 3) as f32;
                x += 1;
            } else {
                // Select S
                d += a2 * (-2 * y + 3) as f32;
            }
            y -= 1;

            self.push_symmetric_points(x, y);
        }
    }

    fn push_symmetric_points(&mut self, x: i32, y: i32) {
        self.points.push([self.centre[0] + x, self.centre[1] + y]);
        self.points.push([self.centre[0] - x, self.centre[1] + y]);
        self.points.push([self.centre[0] + x, self.centre[1] - y]);
        self.points.push([self.centre[0] - x, self.centre[1] - y]);
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
}
