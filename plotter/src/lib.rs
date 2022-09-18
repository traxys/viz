use iced_graphics::{widget::canvas::path::Path as Path2D, Point};
use std::ops::{AddAssign, MulAssign};

pub struct Plotter {
    resolution: usize,
    width: f64,
    height: f64,
    scale: f64,
}

struct Vector2D {
    x: f64,
    y: f64,
}

fn vec2d(x: f64, y: f64) -> Vector2D {
    Vector2D { x, y }
}

impl MulAssign<f64> for Vector2D {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl AddAssign<Vector2D> for Vector2D {
    fn add_assign(&mut self, rhs: Vector2D) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl From<Vector2D> for Point {
    fn from(Vector2D { x, y }: Vector2D) -> Self {
        Point {
            x: x as _,
            y: y as _,
        }
    }
}

fn linspace(p0: f64, p1: f64, count: usize) -> impl Iterator<Item = f64> {
    (0..=count).map(move |x| p0 + x as f64 * (p1 - p0) / (count as f64))
}

impl Plotter {
    pub fn new(resolution: usize, width: f64, height: f64, scale: f64) -> Self {
        Self {
            resolution,
            width: width / scale,
            height: height / scale,
            scale,
        }
    }

    fn screen_coord(&self, mut v: Vector2D) -> Point {
        v.y *= -1.;
        //v += vec2d(self.width / 2., self.height / 2.);
        v *= self.scale;
        v.into()
    }

    pub fn axis(&self) -> Path2D {
        Path2D::new(|axis| {
            axis.move_to(self.screen_coord(vec2d(0., -self.height / 2.)));
            axis.line_to(self.screen_coord(vec2d(0., self.height / 2.)));

            axis.move_to(self.screen_coord(vec2d(-self.width / 2., 0.)));
            axis.line_to(self.screen_coord(vec2d(self.width / 2., 0.)));
        })
    }

    fn clamp_min(&self, x: f64) -> f64 {
        let x_min = -self.width / 2.;
        if x < x_min {
            x_min
        } else {
            x
        }
    }

    fn clamp_max(&self, x: f64) -> f64 {
        let x_max = self.width / 2.;
        if x > x_max {
            x_max
        } else {
            x
        }
    }

    /// `y = a*x^2 + b*x + c`
    pub fn parabola(&self, a: f64, b: f64, c: f64) -> Path2D {
        let edge = if a < 0. { self.height } else { -self.height } / 2.;

        let delta = b * b - 4. * a * (c + edge);
        assert!(
            delta >= 0.,
            "Why should delta be negative, there must be an intersection"
        );

        let x1 = (-b + delta.sqrt()) / (2. * a);
        let x2 = (-b - delta.sqrt()) / (2. * a);
        let (x1, x2) = if x1 < x2 { (x1, x2) } else { (x2, x1) };

        let x1 = self.clamp_min(x1);
        let x2 = self.clamp_max(x2);

        let width = x2 - x1;
        let total_points = (width * self.resolution as f64).ceil() as usize;

        Path2D::new(|path| {
            let mut space = linspace(x1, x2, total_points);
            let first = space.next().unwrap();

            path.move_to(self.screen_coord(vec2d(first, a * first * first + b * first + c)));

            for x in space {
                path.line_to(self.screen_coord(vec2d(x, a * x * x + b * x + c)))
            }
        })
    }
}
