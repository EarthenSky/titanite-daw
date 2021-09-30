use std::f32::consts::PI;
use std::i16;
use hound;

// ------------------------------------------------------------------- //

use iced::{
    button, Button,
    canvas::{self, Cursor, Path, Stroke, Frame, Geometry},
    executor, window, Application, Canvas, Clipboard, Color, Command, Text,
    Element, Column, Length, Point, Rectangle, Settings, Size, Subscription, Vector
};

#[derive(Default)]
struct GUIState {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
    canvasState: CanvasState,
}

// This enumerates all the possible ways we can interact with our ui
#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Application for GUIState {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            GUIState {
                value: 0,
                increment_button: button::State::default(),
                decrement_button: button::State::default(),
                canvasState: CanvasState::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Titanite DAW")
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
                self.canvasState.radius = self.value as f32;
            }
            Message::DecrementPressed => {
                self.value -= 1;
                self.canvasState.radius = self.value as f32;
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

        // We use a column: a simple vertical layout
        Column::new()
            .padding(20)
            .push(
                // The increment button. We tell it to produce an
                // `IncrementPressed` message when pressed
                Button::new(&mut self.increment_button, Text::new("+"))
                    .on_press(Message::IncrementPressed),
            )
            .push(
                // We show the value of the counter here
                Text::new(&self.value.to_string()).size(50),
            )
            .push(
                // The decrement button. We tell it to produce a
                // `DecrementPressed` message when pressed
                Button::new(&mut self.decrement_button, Text::new("-"))
                    .on_press(Message::DecrementPressed),
            )
            .push(
                Canvas::new(&mut self.canvasState),
            )
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
