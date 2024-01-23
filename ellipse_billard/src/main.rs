use iced::{
    executor,
    widget::canvas,
    widget::{
        canvas::{Fill, Stroke, Style},
        slider, Column, Row, Text,
    },
    Application, Color, Command, Length, Theme, Renderer,
};
use plotter::Plotter;

struct EllipseBillard {
    state: State,
}

struct State {
    cache: canvas::Cache,
    eccentricity: f64,
    angle: f64,
    start_offset: f64,
    reflection_count: usize,
}

const RESOLUTION: usize = 100;
const DEFAULT_SCALE: f64 = 100.;

#[derive(Debug, Clone, Copy)]
enum Message {
    Eccentricity(f64),
    Angle(f64),
    StartOffset(f64),
    ReflectionCount(usize),
}

impl Application for EllipseBillard {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                state: State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Billard in an Ellipse".into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Eccentricity(e) => self.state.eccentricity = e,
            Message::Angle(th) => self.state.angle = th,
            Message::StartOffset(s) => self.state.start_offset = s,
            Message::ReflectionCount(r) => self.state.reflection_count = r,
        }
        self.state.cache.clear();

        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        Column::with_children(vec![
            Row::with_children(vec![
                Text::new(format!("Eccentricity ({:.2})", self.state.eccentricity)).into(),
                slider(0.01..=0.99, self.state.eccentricity, |v| {
                    Message::Eccentricity(v)
                })
                .step(0.01)
                .into(),
            ])
            .into(),
            Row::with_children(vec![
                Text::new(format!(
                    "Angle ({:3.2}tau)",
                    self.state.angle / std::f64::consts::TAU
                ))
                .into(),
                slider(0.0..=1., self.state.angle / std::f64::consts::TAU, |v| {
                    Message::Angle(v * std::f64::consts::TAU)
                })
                .step(0.01)
                .into(),
            ])
            .into(),
            Row::with_children(vec![
                Text::new(format!("Starting Offset ({:5.2})", self.state.start_offset)).into(),
                slider(-0.99..=0.99, self.state.start_offset, |v| {
                    Message::StartOffset(v)
                })
                .step(0.01)
                .into(),
            ])
            .into(),
            Row::with_children(vec![
                Text::new(format!(
                    "Reflection Count ({:4})",
                    self.state.reflection_count
                ))
                .into(),
                slider(10.0..=1000., self.state.reflection_count as f64, |v| {
                    Message::ReflectionCount(v as usize)
                })
                .step(1.)
                .into(),
            ])
            .into(),
            canvas(&self.state)
                .width(Length::Fill)
                .height(Length::Fill)
                .into(),
        ])
        .into()
    }
}

impl State {
    fn new() -> Self {
        Self {
            cache: Default::default(),
            eccentricity: 0.8,
            angle: std::f64::consts::FRAC_PI_4,
            start_offset: 0.3,
            reflection_count: 50,
        }
    }
}

impl<Message> canvas::Program<Message> for State {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        vec![self.cache.draw(renderer, bounds.size(), |frame| {
            let iced::Size { width, height } = frame.size();
            let width = width as _;
            let height = height as _;
            let plotter = Plotter::new(RESOLUTION, width, height, DEFAULT_SCALE);

            let (a, b) = plotter::eccentricity_to_radius(self.eccentricity);

            frame.stroke(
                &plotter.centered_ellipse(a, b),
                Stroke::default().with_width(2.0).with_color(Color::BLACK),
            );

            let intersections = |angle: f64, x0: f64, y0: f64| {
                // We choose to either solve with x or y to try and avoid rounding errors
                if angle.sin().abs() >= 0.5 {
                    let angle_factor = angle.cos() / angle.sin();
                    let offset = x0 - angle_factor * y0;

                    let alpha = a * a + (b * angle_factor).powi(2);
                    let beta = 2. * b * b * offset * angle_factor;
                    let gamma = b * b * (offset * offset - a * a);

                    // y coordinate of intersection between ray & ellipse is solution of
                    // alpha * x^2 + beta * x + gamma = 0
                    let delta = beta * beta - 4. * alpha * gamma;
                    assert!(
                        delta >= 0.,
                        "Delta should never be negative due to our choice"
                    );

                    let iy0 = (-beta + (delta).sqrt()) / (2. * alpha);
                    let iy1 = (-beta - (delta).sqrt()) / (2. * alpha);

                    (
                        (iy0 * angle_factor + offset, iy0),
                        (iy1 * angle_factor + offset, iy1),
                    )
                } else {
                    let angle_factor = angle.tan();
                    let offset = y0 - angle_factor * x0;

                    let alpha = b * b + (a * angle_factor).powi(2);
                    let beta = 2. * a * a * offset * angle_factor;
                    let gamma = a * a * (offset * offset - b * b);

                    // x coordinate of intersection between ray & ellipse is solution of
                    // alpha * x^2 + beta * x + gamma = 0
                    let delta = beta * beta - 4. * alpha * gamma;
                    assert!(
                        delta >= 0.,
                        "Delta should never be negative due to our choice"
                    );

                    let ix0 = (-beta + (delta).sqrt()) / (2. * alpha);
                    let ix1 = (-beta - (delta).sqrt()) / (2. * alpha);

                    (
                        (ix0, angle_factor * ix0 + offset),
                        (ix1, angle_factor * ix1 + offset),
                    )
                }
            };

            let angle = self.angle;
            let (sx, sy) = (self.start_offset * a, 0.);

            let ((ix0, iy0), (ix1, iy1)) = intersections(angle, sx, sy);

            let (lower, upper) = if iy0 < 0. {
                ((ix0, iy0), (ix1, iy1))
            } else {
                ((ix1, iy1), (ix0, iy0))
            };

            let first_intersection = if iy0 == 0. {
                if angle == 0. {
                    (ix0.abs(), 0.)
                } else {
                    (-ix0.abs(), 0.)
                }
            } else if angle > std::f64::consts::PI {
                lower
            } else {
                upper
            };

            let tangent_vector = |x0: f64, y0: f64| {
                if y0 == 0. {
                    (0., 1.)
                } else if x0 == 0. {
                    (1., 0.)
                } else {
                    (x0, y0 - (b * b / y0))
                }
            };

            let normal_vector = |x0: f64, y0: f64| {
                let (tx, ty) = tangent_vector(x0, y0);
                let (nx, ny) = (-ty, tx);

                let norm = (nx * nx + ny * ny).sqrt();

                // We want the vector pointing to the center, it's the opposite to the one with
                // the vector to (x0, y0) according to the dot product
                if nx * x0 + ny * y0 >= 0. {
                    (-nx / norm, -ny / norm)
                } else {
                    (nx / norm, ny / norm)
                }
            };

            let outgoing_angle = |sx: f64, sy: f64, x0: f64, y0: f64| {
                let (vx, vy) = (x0 - sx, y0 - sy);
                let (nx, ny) = normal_vector(x0, y0);

                let dot = vx * nx + vy * ny;

                let (ox, oy) = (vx - 2. * dot * nx, vy - 2. * dot * ny);

                oy.atan2(ox)
            };

            let out_th = outgoing_angle(sx, sy, first_intersection.0, first_intersection.1);

            struct Ray {
                start: (f64, f64),
                angle: f64,
            }

            let rays = plotter.path((0..self.reflection_count).scan(
                Ray {
                    start: first_intersection,
                    angle: out_th,
                },
                |r, _| {
                    let (x0, y0) = r.start;

                    let ((ix0, iy0), (ix1, iy1)) = intersections(r.angle, x0, y0);
                    let d0 = (ix0 - x0).abs();
                    let d1 = (ix1 - x0).abs();

                    let (ix, iy) = if d0 > d1 { (ix0, iy0) } else { (ix1, iy1) };

                    r.start = (ix, iy);
                    r.angle = outgoing_angle(x0, y0, ix, iy);
                    assert!(!r.angle.is_nan(), "Outgoing angle is NaN");

                    Some((x0, y0))
                },
            ));

            frame.stroke(
                &rays,
                Stroke::default()
                    .with_width(1.)
                    .with_color(Color::new(1., 0., 0., 1.)),
            );

            frame.stroke(
                &plotter.path([(sx, sy), first_intersection]),
                Stroke::default()
                    .with_width(2.)
                    .with_color(Color::new(0., 0., 1., 1.)),
            );

            frame.fill(
                &plotter.circle(-a * self.eccentricity, 0., 0.05),
                Fill {
                    style: Style::Solid(Color::new(0.3, 0.21, 0.82, 1.)),
                    ..Default::default()
                },
            );
            frame.fill(
                &plotter.circle(a * self.eccentricity, 0., 0.05),
                Fill {
                    style: Style::Solid(Color::new(0.3, 0.21, 0.82, 1.)),
                    ..Default::default()
                },
            );
        })]
    }
}

pub fn main() -> iced::Result {
    #[cfg(target_arch = "wasm32")]
    let platform_specific = {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    
        iced::window::PlatformSpecific {
            target: Some("iced_root".into()),
        }
    };

    #[cfg(not(target_arch = "wasm32"))]
    let platform_specific = iced::window::PlatformSpecific::default();

    EllipseBillard::run(iced::Settings {
        antialiasing: true,
        window: iced::window::Settings {
            platform_specific,
            ..Default::default()
        },
        ..Default::default()
    })
}
