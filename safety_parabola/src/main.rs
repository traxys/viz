use either::Either;
use iced::{
    executor,
    widget::{canvas, canvas::Stroke, pick_list, slider, Column, Row, Text},
    Application, Color, Command, Length, Theme,
};
use plotter::{linspace, Plotter};

const RESOLUTION: usize = 100;
const DEFAULT_SCALE: f64 = 75.;
const EARTH_G: f64 = 9.81;
const DEFAULT_SPACING: ParabolaSpacing = ParabolaSpacing::EqualAngle;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum ParabolaSpacing {
    EqualXIntersect,
    EqualAngle,
}

impl std::fmt::Display for ParabolaSpacing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ParabolaSpacing::EqualXIntersect => "equal x intersect",
            ParabolaSpacing::EqualAngle => "equal angles",
        };
        write!(f, "{}", s)
    }
}

struct SafetyParabola {
    state: State,
}

struct State {
    plot_cache: canvas::Cache,
    v0: f64,
    count: usize,
    spacing: ParabolaSpacing,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    SetV0(f64),
    SetCount(usize),
    SetSpacing(ParabolaSpacing),
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

    SafetyParabola::run(iced::Settings {
        antialiasing: true,
        window: iced::window::Settings {
            platform_specific,
            ..Default::default()
        },
        ..Default::default()
    })
}

impl Application for SafetyParabola {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                state: State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Safety Parabola".into()
    }

    fn update(&mut self, msg: Self::Message) -> iced::Command<Self::Message> {
        match msg {
            Message::SetV0(v0) => self.state.v0 = v0,
            Message::SetCount(c) => self.state.count = c,
            Message::SetSpacing(s) => self.state.spacing = s,
        }
        self.state.plot_cache.clear();

        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        Column::with_children(vec![
            Row::with_children(vec![
                Text::new(format!("count ({:3})", self.state.count)).into(),
                slider(4.0..=100.0, self.state.count as f64, |f: f64| {
                    Message::SetCount(f.ceil() as usize)
                })
                .step(2.0)
                .into(),
            ])
            .into(),
            Row::with_children(vec![
                Text::new(format!("v0 ({:7.2})", self.state.v0)).into(),
                slider(0.1..=20.0, self.state.v0, Message::SetV0)
                    .step(0.1)
                    .into(),
            ])
            .into(),
            Row::with_children(vec![
                Text::new("Spacing").into(),
                pick_list(
                    &[
                        ParabolaSpacing::EqualXIntersect,
                        ParabolaSpacing::EqualAngle,
                    ][..],
                    Some(self.state.spacing),
                    Message::SetSpacing,
                )
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
            plot_cache: canvas::Cache::new(),
            v0: 10.,
            count: 10,
            spacing: DEFAULT_SPACING,
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
        let v0 = self.v0;

        vec![self.plot_cache.draw(bounds.size(), |frame| {
            let iced::Size { width, height } = frame.size();
            let plotter = Plotter::new(RESOLUTION, width as _, height as _, DEFAULT_SCALE);

            let axis = plotter.axis();
            frame.stroke(
                &axis,
                Stroke::default().with_width(2.0).with_color(Color::BLACK),
            );

            let make_parabola = |th: f64| {
                let a = -EARTH_G / (2. * v0 * v0 * th.cos() * th.cos());
                let b = th.tan();

                plotter.parabola(a, b, 0.)
            };

            let x_max = v0 * v0 / EARTH_G;
            let thetas = match self.spacing {
                ParabolaSpacing::EqualXIntersect => Either::Left(
                    linspace(-x_max * 0.95, x_max * 0.95, self.count / 2)
                        .filter(|&x| x != 0.)
                        .flat_map(|x| {
                            let th = (EARTH_G * x / (v0 * v0)).asin() / 2.;
                            [th, th + std::f64::consts::FRAC_PI_2]
                        }),
                ),
                ParabolaSpacing::EqualAngle => Either::Right(
                    linspace(0.001, std::f64::consts::PI, self.count)
                        .filter(|&x| x != std::f64::consts::FRAC_PI_2),
                ),
            };

            for parabola in thetas.map(make_parabola) {
                frame.stroke(
                    &parabola,
                    Stroke::default().with_width(3.0).with_color(Color::BLACK),
                )
            }

            let safety_parabola =
                plotter.parabola(-EARTH_G / (2. * v0 * v0), 0., v0 * v0 / (2. * EARTH_G));

            frame.stroke(
                &safety_parabola,
                Stroke::default()
                    .with_width(3.0)
                    .with_color(Color::from_rgb(1., 0., 0.)),
            )
        })]
    }
}
