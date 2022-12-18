pub struct Circle {
    points: Vec<[i32; 2]>,
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
        self.points.push([self.centre[0] + x, self.centre[1] + y]);
        self.points.push([self.centre[0] - x, self.centre[1] + y]);
        self.points.push([self.centre[0] + x, self.centre[1] - y]);
        self.points.push([self.centre[0] - x, self.centre[1] - y]);
        self.points.push([self.centre[0] + y, self.centre[1] + x]);
        self.points.push([self.centre[0] - y, self.centre[1] + x]);
        self.points.push([self.centre[0] + y, self.centre[1] - x]);
        self.points.push([self.centre[0] - y, self.centre[1] - x]);
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
}
