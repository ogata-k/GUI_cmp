use iced::{Sandbox, Element, Button, Column, Text, Settings, Container, Length, Align, HorizontalAlignment, Color, Background, button};
use iced::settings::Window;

pub fn main() {
    Counter::run(Settings {
        window: Window {
            size: (300, 300), // (x, y)
            resizable: false,
        }
    })
}

// state
struct Counter {
    // counter value
    value: i32,

    // state of the two buttons
    increment_button: button::State,
    reset_button: button::State,
}

// message
#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Reset,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Counter {
            value: 0,
            increment_button: button::State::default(),
            reset_button: button::State::default(),
        }
    }

    fn title(&self) -> String {
        String::from("count up")
    }

    // update
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Reset => {
                self.value = 0;
            }
        }
    }

    // view logic
    fn view(&mut self) -> Element<Message> {
        Container::new(
            Column::new()
                .push(
                    Button::new(&mut self.increment_button, Text::new("+1"))
                        .on_press(Message::Increment)
                        .border_radius(5)
                        .background(Background::Color(Color{r: 0.8, g: 0.8, b: 0.8, a: 1.})),
                )
                .push(
                    Text::new(self.value.to_string()).size(50).horizontal_alignment(HorizontalAlignment::Center),
                )
                .push(
                    Button::new(&mut self.reset_button, Text::new("reset"))
                        .on_press(Message::Reset),
                )
                .align_items(Align::Center)
        )
            .width(Length::Fill)
            .center_x()
            .height(Length::Fill)
            .center_y()
            .into()
    }
}
