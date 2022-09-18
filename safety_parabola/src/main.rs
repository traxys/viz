use iced::{canvas, executor, window, Application, Command};
use plotter::Plotter;

const RESOLUTION: usize = 100;
const SCALE: f64 = 10.;

struct SafetyParabola {
    state: State,
}

struct State {
    plot_cache: canvas::Cache,
    plotter: Plotter,
}

pub fn main() -> iced::Result {
    SafetyParabola::run(iced::Settings {
        antialiasing: true,
        ..Default::default()
    })
}

impl Application for SafetyParabola {
    type Executor = executor::Default;
    type Message = ();
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

    fn update(&mut self, _: Self::Message) -> iced::Command<Self::Message> {
        todo!()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        todo!()
    }
}

impl State {
    fn new() -> Self {
        let (width, height) = window::Settings::default().size;

        Self {
            plot_cache: canvas::Cache::new(),
            plotter: Plotter::new(RESOLUTION, width as _, height as _, SCALE),
        }
    }
}
