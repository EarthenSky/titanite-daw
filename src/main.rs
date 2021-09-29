use std::f32::consts::PI;
use std::i16;
use hound;

// ------------------------------------------------------------------- //

use iced::{button, Button, Column, Element, Sandbox, Settings, Text};

#[derive(Default)]
struct Counter {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
}

// This enumerates all the possible ways we can interact with our ui
#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }

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
            .into()
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

    println!("{}", match Counter::run(Settings::default()) {
        Error => "not good not good not good",
        _ => "we all good",
    });

}

// ------------------------------------------------------------------- //

