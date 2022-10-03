use std::collections::HashMap;

use iced::{
    executor,
    widget::{canvas, canvas::Stroke, checkbox, slider, Column, Row, Text},
    Application, Color, Command, Length, Theme,
};
use palette::{rgb::Rgb, FromColor, Lch, Srgb};
use petgraph::graph::Graph;
use plotter::Plotter;

struct ModularTable {
    state: State,
}

struct State {
    cache: canvas::Cache,
    modulo: u64,
    scale: f64,
    multiplier: u64,
    colored: bool,
    label: bool,
}

const RESOLUTION: usize = 100;
const DEFAULT_SCALE: f64 = 180.;

#[derive(Debug, Clone, Copy)]
enum Message {
    Modulo(u64),
    Scale(f64),
    Multiplier(u64),
    Colored(bool),
    Labeled(bool),
}

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
        match message {
            Message::Modulo(n) => {
                self.state.modulo = n;
                if self.state.multiplier >= n {
                    self.state.multiplier = n - 1;
                }
            }
            Message::Scale(s) => self.state.scale = s,
            Message::Multiplier(m) => self.state.multiplier = m,
            Message::Colored(b) => self.state.colored = b,
            Message::Labeled(l) => self.state.label = l,
        }
        self.state.cache.clear();

        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        Column::with_children(vec![
            Row::with_children(vec![
                Text::new(format!("Modulo ({:4})", self.state.modulo)).into(),
                slider(3.0..=300.0, self.state.modulo as _, |v| {
                    Message::Modulo(v as _)
                })
                .into(),
            ])
            .into(),
            Row::with_children(vec![
                Text::new(format!("Multiplier ({:4})", self.state.multiplier)).into(),
                slider(
                    2.0..=((self.state.modulo - 1) as _),
                    self.state.multiplier as _,
                    |v| Message::Multiplier(v as _),
                )
                .into(),
            ])
            .into(),
            Row::with_children(vec![
                Text::new(format!("Scale ({:4.2})", self.state.scale)).into(),
                slider(1.0..=6., self.state.scale, Message::Scale)
                    .step(0.1)
                    .into(),
            ])
            .into(),
            Row::with_children(vec![
                checkbox(
                    "Colors for components",
                    self.state.colored,
                    Message::Colored,
                )
                .into(),
                checkbox("Labels on the nodes", self.state.label, Message::Labeled).into(),
            ])
            .into(),
            canvas(&self.state)
                .width(Length::Fill)
                .height(Length::Fill)
                .into(),
        ])
        .padding(5)
        .into()
    }
}

impl State {
    fn new() -> Self {
        Self {
            cache: canvas::Cache::new(),
            modulo: 10,
            multiplier: 2,
            scale: 1.,
            colored: false,
            label: true,
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
            let plotter = Plotter::new(
                RESOLUTION,
                width as _,
                height as _,
                self.scale * DEFAULT_SCALE,
            );

            frame.stroke(
                &plotter.centered_circle(1.),
                Stroke::default().with_width(3.0).with_color(Color::BLACK),
            );

            let step = std::f64::consts::TAU / self.modulo as f64;
            let coord = |i| {
                let angle = std::f64::consts::FRAC_PI_2 - (i as f64 * step);
                let x = angle.cos();
                let y = angle.sin();

                (x, y)
            };

            let mut graph = Graph::<u64, (), _>::new_undirected();
            let mut nodes = HashMap::new();

            for i in 1..=self.modulo {
                let (x, y) = coord(i);

                let r = (i * self.multiplier) % self.modulo;
                let r = if r == 0 { self.modulo } else { r };

                let &mut i_idx = nodes.entry(i).or_insert_with(|| graph.add_node(i));
                let &mut r_idx = nodes.entry(r).or_insert_with(|| graph.add_node(r));
                graph.add_edge(i_idx, r_idx, ());

                frame.fill(
                    &plotter.circle(x, y, 0.03),
                    canvas::Fill {
                        color: Color::BLACK,
                        ..Default::default()
                    },
                );

                if self.label {
                    frame.fill_text(plotter.text(x * 1.1, y * 1.1, i.to_string()));
                }
            }

            let components = petgraph::algo::kosaraju_scc(&graph);
            let cn = components.len() as f64;

            for (i, component) in components.iter().enumerate() {
                let lch = Lch::new(80., 100., (i as f64 / cn) * 360.);
                let Rgb {
                    red, green, blue, ..
                } = Srgb::from_color(lch);

                let stroke = if self.colored {
                    Stroke::default().with_color(Color::from_rgb(
                        red as f32,
                        green as f32,
                        blue as f32,
                    ))
                } else {
                    Stroke::default().with_color(Color::from_rgb(1.0, 0.0, 0.0))
                };

                for (i, &a) in component.iter().take(component.len() - 1).enumerate() {
                    for &b in &component[i + 1..] {
                        if graph.contains_edge(a, b) {
                            frame.stroke(
                                &plotter.path([
                                    coord(*graph.node_weight(a).unwrap()),
                                    coord(*graph.node_weight(b).unwrap()),
                                ]),
                                stroke,
                            );
                        }
                    }
                }
            }
        })]
    }
}
