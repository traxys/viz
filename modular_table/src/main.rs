use std::collections::HashMap;

use iced::{
    executor,
    widget::canvas,
    widget::{
        canvas::{Stroke, Style},
        checkbox, slider, Column, Row, Text,
    },
    Application, Color, Command, Length, Theme, Renderer,
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
    arrow: bool,
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
    Arrow(bool),
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

    ModularTable::run(iced::Settings {
        antialiasing: true,
        window: iced::window::Settings {
            platform_specific,
            ..Default::default()
        },
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
            Message::Arrow(a) => self.state.arrow = a,
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
                checkbox("Arrows", self.state.arrow, Message::Arrow).into(),
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
            arrow: false,
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
            let mut graph_directed = Graph::<u64, (), _>::new();
            let mut nodes_directed = HashMap::new();

            for i in 1..=self.modulo {
                let (x, y) = coord(i);

                let r = (i * self.multiplier) % self.modulo;
                let r = if r == 0 { self.modulo } else { r };

                let &mut i_idx = nodes.entry(i).or_insert_with(|| graph.add_node(i));
                let &mut r_idx = nodes.entry(r).or_insert_with(|| graph.add_node(r));
                graph.add_edge(i_idx, r_idx, ());

                let &mut i_idx = nodes_directed
                    .entry(i)
                    .or_insert_with(|| graph_directed.add_node(i));
                let &mut r_idx = nodes_directed
                    .entry(r)
                    .or_insert_with(|| graph_directed.add_node(r));
                graph_directed.add_edge(i_idx, r_idx, ());

                frame.fill(
                    &plotter.circle(x, y, 0.03),
                    canvas::Fill {
                        style: Style::Solid(Color::BLACK),
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
                            let &a = graph.node_weight(a).unwrap();
                            let &b = graph.node_weight(b).unwrap();
                            if self.arrow {
                                let ia = nodes_directed[&a];
                                let ib = nodes_directed[&b];
                                let (start, end) = if graph_directed.contains_edge(ia, ib) {
                                    (a, b)
                                } else {
                                    (b, a)
                                };
                                frame.stroke(
                                    &plotter.arrow_absolute_size(coord(start), coord(end), 0.05),
                                    stroke.clone(),
                                );
                            } else {
                                frame.stroke(&plotter.path([coord(a), coord(b)]), stroke.clone());
                            }
                        }
                    }
                }
            }
        })]
    }
}
