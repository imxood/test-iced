#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::{
    canvas::{Cursor, Frame, Geometry, Program},
    executor, Align, Application, Canvas, Clipboard, Color, Column, Container, Element, Length,
    Point, Row, Size, Text,
};
use iced::{Command, Settings};
use iced_aw::{split, Split};

fn main() -> iced::Result {
    MyApp::run(Settings::default())
}

struct MyApp {
    split_pane: split::State,
}

#[derive(Debug, Clone)]
enum Message {
    OnResize(u16),
}

impl Application for MyApp {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            Self {
                split_pane: split::State::new(Some(100), split::Axis::Vertical),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        match message {
            Message::OnResize(position) => self.split_pane.set_divider_position(position),
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let canvas = Canvas::new(Grid::new())
            .width(Length::Fill)
            .height(Length::Fill);

        let control = Column::<Message>::new()
            .max_width(400)
            .spacing(20)
            .push(Text::new("hello, the world").size(50));

        let first = Container::new(canvas)
            .width(Length::Fill)
            .height(Length::Fill);

        let second = Container::new(control)
            .width(Length::FillPortion(400))
            .height(Length::Fill);

        let split: Element<_> =
            Split::new(&mut self.split_pane, first, second, Message::OnResize).into();

        split.explain(Color::from_rgb8(0x00, 0x00, 0xff))
    }
}

struct Grid {
    row: i32,
    col: i32,
    node_size: f32,
    interval: f32,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            row: 120,
            col: 200,
            node_size: 3.,
            interval: 1.,
        }
    }
}

impl Program<Message> for Grid {
    fn draw(&self, bounds: iced::Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let mut frame = Frame::new(bounds.size());
        for row in 0..self.row {
            let y_start = (self.node_size + self.interval) * (row + 1) as f32;
            for col in 0..self.col {
                let x_start = (self.node_size + self.interval) * (col + 1) as f32;
                frame.fill_rectangle(
                    Point::new(x_start, y_start),
                    Size::new(self.node_size, self.node_size),
                    Color::from_rgb8(0xff, 0x00, 0x00),
                );
            }
        }
        vec![frame.into_geometry()]
    }
}
