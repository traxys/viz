use iced::{
    executor,
    widget::{canvas, canvas::Stroke, slider, Column, Row, Text},
    Application, Color, Command, Length, Theme,
};
use plotter::{linspace, Plotter};

const RESOLUTION: usize = 100;
const DEFAULT_SCALE: f64 = 75.;
const EARTH_G: f64 = 9.81;

struct SafetyParabola {
    state: State,
}

struct State {
    plot_cache: canvas::Cache,
    v0: f64,
    scale: f64,
    count: usize,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    SetV0(f64),
    SetScale(f64),
    SetCount(usize),
}

pub fn main() -> iced::Result {
    SafetyParabola::run(iced::Settings {
        antialiasing: true,
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
            Message::SetScale(scale) => self.state.scale = scale,
            Message::SetCount(c) => self.state.count = c,
        }
        self.state.plot_cache.clear();

        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        Column::with_children(vec![
            Row::with_children(vec![
                Text::new(format!("count ({:3})", self.state.count)).into(),
                slider(1.0..=100.0, self.state.count as f64, |f: f64| {
                    Message::SetCount(f.ceil() as usize)
                })
                .into(),
            ])
            .into(),
            Row::with_children(vec![
                Text::new(format!("v0 ({:7.2})", self.state.v0)).into(),
                slider(1.0..=1000., self.state.v0, Message::SetV0).into(),
            ])
            .into(),
            Row::with_children(vec![
                Text::new(format!("scale ({:7.2})", self.state.scale)).into(),
                slider(0.1..=100., self.state.scale, Message::SetScale)
                    .step(0.1)
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
            scale: DEFAULT_SCALE,
            count: 10,
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
            let plotter = Plotter::new(RESOLUTION, width as _, height as _, self.scale);

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
            let parabolas = linspace(-x_max * 0.95, x_max * 0.95, self.count / 2)
                .filter(|&x| x != 0.)
                .flat_map(|x| {
                    let th = (EARTH_G * x / (v0 * v0)).asin() / 2.;
                    [th, th + std::f64::consts::FRAC_PI_2]
                })
                .map(make_parabola);

            for parabola in parabolas {
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
