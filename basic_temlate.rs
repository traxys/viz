use iced::{executor, widget::canvas, Application, Command, Length, Theme};
use plotter::Plotter;

struct ModularTable {
    state: State,
}

struct State {
    cache: canvas::Cache,
    modulo: u64,
    multiplier: u64,
}

const RESOLUTION: usize = 100;
const DEFAULT_SCALE: f64 = 75.;

#[derive(Debug)]
enum Message {}

pub fn main() -> iced::Result {
    ModularTable::run(iced::Settings {
        antialiasing: true,
        ..Default::default()
    })
}

impl Application for ModularTable {
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
        "Modular Table".into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {}
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        canvas(&self.state)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl State {
    fn new() -> Self {
        Self {
            cache: canvas::Cache::new(),
            modulo: 10,
            multiplier: 2,
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
            let _plotter = Plotter::new(RESOLUTION, width as _, height as _, DEFAULT_SCALE);
        })]
    }
}
