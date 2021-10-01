use std::f32::consts::PI;
use std::i16;
use hound;

use iced::{
    button, Button,
    canvas::{self, Cursor, Path, Frame, Geometry}, Canvas, 
    widget::{Container},
    Length, Point, Size, 
    executor, Application, Clipboard, Command, Settings, Subscription,
    Color, Text,
    Element, Column, Row, 
    Rectangle,  
};

// Declaring modules:
mod model;
mod colors;

// ------------------------------------------------------------------- //

#[derive(Default)]
struct GUIState {
    value: i32,
    settings_button: button::State,
    canvas_state: CanvasState,
}

// This enumerates all the possible ways we can interact with our ui
#[derive(Debug, Clone, Copy)]
pub enum Message {
    ShowSettings,
}

impl Application for GUIState {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            GUIState {
                value: 0,
                settings_button: button::State::default(),
                canvas_state: CanvasState::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Titanite DAW")
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::ShowSettings => {
                println!("TODO: open settings window ...");
            }
        }

        Command::none()
    }

    /*
    fn subscription(&self) -> Subscription<Message> {
        time::every(std::time::Duration::from_millis(10))
            .map(|instant| Message::Tick(instant))
    }
    */

    fn view(&mut self) -> Element<Message> {

        let menu_bar = Container::new(
            Row::new()
            .padding(0)
            .push(
                Button::new(&mut self.settings_button, Text::new("Settings"))
                .on_press(Message::ShowSettings).style(colors::Button)
            )
        )
        .width(Length::Fill)
        .style(colors::Container);
 
        let canvas = Canvas::new(&mut self.canvas_state)
            .width(Length::Fill)
            .height(Length::Fill);

        Column::new()
            .push(menu_bar)
            .push(canvas)
            .into()
    }

}


// First, we define the data we need for drawing
#[derive(Debug)]
struct CanvasState {
    radius: f32,
}

impl Default for CanvasState {
    fn default() -> CanvasState {
        CanvasState { radius: 5.0 }
    }
}

// Then, we implement the `Program` trait
impl canvas::Program<Message> for CanvasState {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let mut frame = Frame::new(bounds.size());

        frame.fill_rectangle(Point::new(0.0, 0.0), frame.size(), Color::new(1.0, 1.0, 0.6, 1.0));

        // We create a `Path` representing a simple circle & fill it
        let circle = Path::circle(frame.center(), self.radius);
        frame.fill(&circle, Color::BLACK);

        // Finally, we produce the geometry
        vec![frame.into_geometry()]
    }
}


// ------------------------------------------------------------------- //

fn main() {
    println!("Hello, world!");

    // generate wav file
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    
    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();
    let s: u32 = 3;
    for t in (0 .. s * 44100).map(|x| x as f32 / 44100.0) {
        let sample = (t * 440.0 * 2.0 * PI).sin(); // our sample is [0, 1]
        let amplitude = i16::MAX as f32;
        let int_sample = (sample * amplitude) as i16; // we scale our sample to 
        writer.write_sample(int_sample).unwrap();
    }

    // Run Reactive GUI
    println!("{}", match GUIState::run(Settings::default()) {
        Err(_e) => "not good not good not good",
        Ok(_) => "we all good",
    });

}

// ------------------------------------------------------------------- //
