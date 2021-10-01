use iced::{
    keyboard,
    canvas::{self, Cursor, Path, Frame, Geometry}, Canvas, 
    Point, Size,
    Color, 
    Rectangle,  
};

use crate::Message;
use crate::colors::{Dark2, DarkFull};

// ---------------------------------------------------------------------- //

// TODO: change this to just "State"?
#[derive(Debug)]
pub struct CanvasState {
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

        // TODO: get an immutable reference from our model here, but throw it away after.

        frame.fill_rectangle(Point::new(0.0, 0.0), frame.size(), Dark2);

        // We create a `Path` representing a simple circle & fill it
        let circle = Path::circle(frame.center(), self.radius);
        frame.fill(&circle, DarkFull);

        // Finally, we produce the geometry
        vec![frame.into_geometry()]
    }

    fn update(&mut self, event: canvas::Event, _bounds: Rectangle<f32>, _cursor: Cursor) -> (canvas::event::Status, Option<Message>) {
        (
            match event {
                canvas::event::Event::Keyboard(event) => match event {
                    keyboard::Event::KeyPressed{key_code, modifiers} => {
                        println!("KeyPressed Event -> {:?} {:?}", key_code, modifiers);
                        canvas::event::Status::Captured
                    },
                    _ => canvas::event::Status::Ignored,
                },
                canvas::event::Event::Mouse(_) => canvas::event::Status::Ignored,
            }, 
            None,
        )
    }
}