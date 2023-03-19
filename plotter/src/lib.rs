use iced_graphics::{
    alignment::{Horizontal, Vertical},
    widget::canvas::{
        path::{arc::Elliptical, Path as Path2D},
        Text,
    },
    Point, Vector,
};
use std::{
    f64::consts::FRAC_PI_6,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

pub fn eccentricity_to_radius(e: f64) -> (f64, f64) {
    let a = 1. / (1. - e * e).sqrt();
    (a, 1.)
}

pub struct Plotter {
    resolution: usize,
    width: f64,
    height: f64,
    scale: f64,
}

#[derive(Clone, Copy)]
pub struct Vector2D {
    x: f64,
    y: f64,
}

impl Vector2D {
    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Self {
        *self / self.norm()
    }

    pub fn rot(&self, angle: f64) -> Vector2D {
        Vector2D {
            x: self.x * angle.cos() - self.y * angle.sin(),
            y: self.x * angle.sin() + self.y * angle.cos(),
        }
    }
}

impl From<(f64, f64)> for Vector2D {
    fn from((x, y): (f64, f64)) -> Self {
        vec2d(x, y)
    }
}

pub fn vec2d(x: f64, y: f64) -> Vector2D {
    Vector2D { x, y }
}

macro_rules! assign_impl {
    ($($op_name:ident=$op:tt);*;) => {
        $(
        paste::paste!{
            impl [< $op_name:camel Assign>]<Vector2D> for Vector2D {
                fn [< $op_name _assign >](&mut self, rhs: Vector2D) {
                    self.x $op rhs.x;
                    self.y $op rhs.y;
                }
            }
        }
        )*
    };
}

macro_rules! op_impl {
    ($($op_name:ident=$op:tt);*;) => {
        $(
        paste::paste!{
            impl [< $op_name:camel >]<Vector2D> for Vector2D {
                type Output = Vector2D;
                fn $op_name(self, rhs: Vector2D) -> Vector2D {
                    Vector2D {
                        x: self.x $op rhs.x,
                        y: self.y $op rhs.y,
                    }
                }
            }
        }
        )*
    };
}

macro_rules! scalar_op_assign {
    ($($op_name:ident=$op:tt);* ;) => {
        $(
        paste::paste! {
            impl [< $op_name:camel Assign>]<f64> for Vector2D {
                fn [< $op_name _assign >](&mut self, rhs: f64) {
                    self.x $op rhs;
                    self.y $op rhs;
                }
            }
        }
        )*
    };
}

macro_rules! scalar_op {
    ($($op_name:ident=$op:tt);* ;) => {
        $(
        paste::paste! {
            impl [< $op_name:camel >]<f64> for Vector2D {
                type Output = Vector2D;
                fn $op_name(self, rhs: f64) -> Vector2D {
                    Vector2D {
                        x: self.x $op rhs,
                        y: self.y $op rhs,
                    }
                }
            }
        }
        )*
    };
}

assign_impl! {
    sub = -=;
    add = +=;
}

op_impl! {
    add = +;
    sub = -;
}

scalar_op_assign! {
    mul = *=;
    div = /=;
}

scalar_op! {
    mul = *;
    div = /;
}

impl From<Vector2D> for Point {
    fn from(Vector2D { x, y }: Vector2D) -> Self {
        Point {
            x: x as _,
            y: y as _,
        }
    }
}

pub fn linspace(p0: f64, p1: f64, count: usize) -> impl Iterator<Item = f64> {
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
        v += vec2d(self.width / 2., self.height / 2.);
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

    ///
    /// The head_size is from 0 to 1. 1 means that the head is the same length as the arrow body,
    /// and 0 means the mead is of size 0. It is a linear interpolation between the two.
    ///
    pub fn arrow<C>(&self, start: C, end: C, head_size: f64) -> Path2D
    where
        C: Into<Vector2D>,
    {
        let start = start.into();
        let end = end.into();

        let mut vec = end - start;

        vec *= head_size * 2. / 3f64.sqrt();

        let arrow = vec.rot(FRAC_PI_6);
        let base = arrow.rot(FRAC_PI_6 * 2.);

        Path2D::new(|builder| {
            builder.move_to(self.screen_coord(start));
            builder.line_to(self.screen_coord(start + (end - start) * (1. - head_size)));
            builder.move_to(self.screen_coord(end));
            builder.line_to(self.screen_coord(end - arrow));
            builder.line_to(self.screen_coord(end - arrow + base));
            builder.line_to(self.screen_coord(end));
        })
    }

    ///
    /// This is the same as Plotter::arrow, but the head_size is in absolute pixels
    ///
    pub fn arrow_absolute_size<C>(&self, start: C, end: C, head_size: f64) -> Path2D
    where
        C: Into<Vector2D>,
    {
        let start = start.into();
        let end = end.into();

        self.arrow(start, end, head_size / (end - start).norm())
    }

    pub fn path<I, C>(&self, parts: I) -> Path2D
    where
        C: Into<Vector2D>,
        I: IntoIterator<Item = C>,
    {
        Path2D::new(|builder| {
            let mut coords = parts
                .into_iter()
                .map(Into::into)
                .map(|c| self.screen_coord(c));

            let first = match coords.next() {
                None => return,
                Some(f) => f,
            };

            builder.move_to(first);
            for point in coords {
                builder.line_to(point);
            }
        })
    }

    pub fn circle(&self, x: f64, y: f64, radius: f64) -> Path2D {
        Path2D::new(|builder| {
            builder.circle(
                self.screen_coord(vec2d(x, y)),
                radius as f32 * self.scale as f32,
            );
        })
    }

    pub fn ellipse(&self, x: f64, y: f64, a: f64, b: f64) -> Path2D {
        Path2D::new(|builder| {
            builder.ellipse(Elliptical {
                center: self.screen_coord(vec2d(x, y)),
                radii: Vector::new((a * self.scale) as _, (b * self.scale) as _),
                rotation: 0.,
                start_angle: 0.,
                end_angle: std::f32::consts::TAU,
            })
        })
    }

    pub fn centered_ellipse(&self, a: f64, b: f64) -> Path2D {
        self.ellipse(0., 0., a, b)
    }
    pub fn centered_circle(&self, radius: f64) -> Path2D {
        self.circle(0., 0., radius)
    }

    pub fn text(&self, x: f64, y: f64, content: String) -> Text {
        let position = self.screen_coord(vec2d(x, y));

        Text {
            content,
            position,
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
            ..Default::default()
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

        self.function(x1, x2, |x| a * x * x + b * x + c)
    }

    pub fn function<F>(&self, start: f64, end: f64, f: F) -> Path2D
    where
        F: Fn(f64) -> f64,
    {
        let start = self.clamp_min(start);
        let end = self.clamp_max(end);

        let width = end - start;
        let total_points = (width * self.resolution as f64).ceil() as usize;

        let point = |x| self.screen_coord(vec2d(x, f(x)));

        Path2D::new(|path| {
            let mut space = linspace(start, end, total_points);
            let first = space.next().unwrap();

            path.move_to(point(first));

            for x in space {
                path.line_to(point(x))
            }
        })
    }
}
