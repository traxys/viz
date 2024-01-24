use iced::{
    executor,
    widget::{canvas, slider, Column, Text},
    widget::{canvas::Stroke, Row},
    Application, Color, Command, Length, Theme,
};
use plotter::Plotter;

struct BicycleMonoTrack {
    state: State,
}

struct State {
    cache: canvas::Cache,
    curve_scale: f64,
    segment_count: usize,
    smoothing_window: usize,
    canvas_scale: f64,
    translation: f64,
}

const RESOLUTION: usize = 100;
const DEFAULT_SCALE: f64 = 150.;

#[derive(Debug, Clone, Copy)]
enum Message {
    CanvasScale(f64),
    CurveScale(f64),
    SegmentCount(usize),
    SmoothingWindowSize(usize),
    Transaltion(f64),
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

    BicycleMonoTrack::run(iced::Settings {
        antialiasing: true,
        window: iced::window::Settings {
            platform_specific,
            ..Default::default()
        },
        ..Default::default()
    })
}

impl Application for BicycleMonoTrack {
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
        "Mono Bicycle Track".into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::CanvasScale(s) => self.state.canvas_scale = s,
            Message::CurveScale(f) => self.state.curve_scale = f,
            Message::SegmentCount(c) => self.state.segment_count = c,
            Message::SmoothingWindowSize(s) => self.state.smoothing_window = s,
            Message::Transaltion(t) => self.state.translation = t,
        }
        self.state.cache.clear();

        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        Column::with_children(vec![
            Row::with_children(vec![
                Text::new(format!("Curve Scale ({:3})", self.state.curve_scale / 50.)).into(),
                slider(1.0..=200.0, self.state.curve_scale / 50., |v| {
                    Message::CurveScale(v * 50.)
                })
                .into(),
            ])
            .into(),
            Row::with_children(vec![
                Text::new(format!("Segment Count ({:1})", self.state.segment_count,)).into(),
                slider(1.0..=9.0, self.state.segment_count as _, |v| {
                    Message::SegmentCount(v as _)
                })
                .into(),
            ])
            .into(),
            Row::with_children(vec![
                Text::new(format!(
                    "Canvas Scale ({:3.2})",
                    self.state.canvas_scale / DEFAULT_SCALE,
                ))
                .into(),
                slider(0.2..=1.0, self.state.canvas_scale / DEFAULT_SCALE, |v| {
                    Message::CanvasScale(v * DEFAULT_SCALE)
                })
                .step(0.05)
                .into(),
            ])
            .into(),
            Row::with_children(vec![
                Text::new(format!("Translation ({:3.2})", self.state.translation,)).into(),
                slider(0.0..=1.0, self.state.translation, |v| {
                    Message::Transaltion(v)
                })
                .step(0.05)
                .into(),
            ])
            .into(),
            Row::with_children(vec![
                Text::new(format!(
                    "Smoothing Window ({:2})",
                    self.state.smoothing_window,
                ))
                .into(),
                slider(1.0..=200.0, self.state.smoothing_window as _, |v| {
                    Message::SmoothingWindowSize(v as _)
                })
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
            cache: canvas::Cache::new(),
            curve_scale: 50.,
            segment_count: 4,
            smoothing_window: 10,
            canvas_scale: DEFAULT_SCALE,
            translation: 0.0,
        }
    }
}

impl<Message> canvas::Program<Message> for State {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        _theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: canvas::Cursor,
    ) -> Vec<canvas::Geometry> {
        vec![self.cache.draw(bounds.size(), |frame| {
            let iced::Size { width, height } = frame.size();
            let width = width as _;
            let height = height as _;
            let plotter = Plotter::new(RESOLUTION, width, height, self.canvas_scale);

            let flat_at = |x: f64, flat: f64| {
                if x == flat {
                    0.
                } else {
                    (-1. / ((flat - x) * (flat - x))).exp()
                }
            };

            let f = |x| self.curve_scale * flat_at(x, 0.) * flat_at(x, 1.);
            let fp = |x| {
                if x == 0. || x == 1. {
                    0.
                } else {
                    f(x) * 2. * (1. / (x * x * x) + 1. / (x - 1.) * (x - 1.) * (x - 1.))
                }
            };

            frame.translate([(-(1. - self.translation) * (width / 2.)) as f32, 0.].into());

            let f_plot = plotter.function(0.0, 1., f);
            frame.stroke(&f_plot, Stroke::default().with_color(Color::BLACK));

            let resulting: Vec<_> = (0..1_000_000)
                .map(|x| x as f64 / 1_000_000.)
                .map(|x| {
                    let v = f(x);
                    let d = fp(x);
                    let norm = (1. + d * d).sqrt();
                    let (tx, ty) = (1. / norm, d / norm);

                    (x + tx, v + ty)
                })
                .collect();

            let second_path = plotter.path(resulting.iter().copied());
            frame.stroke(&second_path, Stroke::default().with_color(Color::BLACK));

            let path = |previous: Vec<(f64, f64)>| -> Vec<_> {
                previous
                    .iter()
                    .zip(previous.iter().skip(1))
                    .map(|((x0, y0), (x1, y1))| {
                        let x = x1 - x0;
                        let y = y1 - y0;
                        let norm = (x * x + y * y).sqrt();
                        let (tx, ty) = (x / norm, y / norm);

                        (x0 + tx, y0 + ty)
                    })
                    .collect()
            };

            let mut current_path = resulting;
            for _ in 0..(self.segment_count - 2) {
                let mut next_path = path(current_path);

                let window = self.smoothing_window;
                for i in window..(next_path.len() - window) {
                    let (x, y) = next_path[i - window..=i + window]
                        .iter()
                        .fold((0., 0.), |(xs, ys), (x, y)| (xs + x, ys + y));
                    let count = window as f64 * 2. + 1.;
                    next_path[i] = (x / count, y / count);
                }

                frame.stroke(
                    &plotter.path(next_path.iter().copied()),
                    Stroke::default().with_color(Color::BLACK),
                );
                current_path = next_path;
            }
        })]
    }
}
