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
        let canvas = Column::<Message>::new()
            .width(Length::FillPortion(70))
            .height(Length::Fill)
            .spacing(20)
            .push(
                Canvas::new(Grid::new())
                    .width(Length::Fill)
                    .height(Length::Fill),
            );

        let control = Column::<Message>::new()
            .width(Length::FillPortion(30))
            .height(Length::Fill)
            .spacing(20)
            .push(Text::new("hello, the world").size(50));

        let row = Row::<Message>::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(canvas)
            .push(control)
            .align_items(Align::End);

        let container: Element<_> = Container::new(row)
            .width(Length::Fill)
            .height(Length::Fill)
            .into();

        // container.explain(Color::from_rgb8(0x00, 0x00, 0xff))
        container
    }
}

struct Grid {
    row: i32,
    col: i32,
    interval: f32,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            row: 32,
            col: 20,
            interval: 3.,
        }
    }
}

impl Program<Message> for Grid {
    fn draw(&self, bounds: iced::Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let size = bounds.size();

        let mut x_offset = 0.;
        let mut y_offset = 0.;
        let node_size;

        if size.width / size.height > self.col as f32 / self.row as f32 {
            // 表示 canvas 的 width 比较大, 小的(即height)则铺满, x 方向有偏移
            node_size = (size.height + self.interval) / self.row as f32 - self.interval;
            x_offset = (size.width - (node_size  + self.interval) * self.col as f32) / 2.0;
        } else {
            // 表示 canvas 的 height 比较大, 小的(即width)则铺满, y 方向有偏移
            node_size = (size.width + self.interval) / self.col as f32 - self.interval;
            y_offset = (size.height - (node_size  + self.interval) * self.row as f32) / 2.0;
        };

        let mut frame = Frame::new(size);
        for row in 0..self.row {
            let y_start = y_offset + (node_size + self.interval) * row as f32;
            for col in 0..self.col {
                let x_start = x_offset + (node_size + self.interval) * col as f32;
                frame.fill_rectangle(
                    Point::new(x_start, y_start),
                    Size::new(node_size, node_size),
                    Color::from_rgb8(0xff, 0xd7, 0x00),
                );
            }
        }
        vec![frame.into_geometry()]
    }
}
