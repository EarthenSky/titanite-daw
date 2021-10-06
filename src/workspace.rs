use iced::{
    keyboard::{self, KeyCode},
    canvas::{self, event::Status, Cursor, Path, Frame, Geometry}, Canvas, 
    canvas::event::Event::Keyboard,
    canvas::event::Event::Mouse,
    Point, Size,
    Color, 
    Rectangle,
};

use crate::Message;
use crate::colors::{Dark2, DarkFull, Light1};
use crate::model::{Model, NoteBlock};

// ---------------------------------------------------------------------- //

// TODO: change this to just "State"?
#[derive(Debug)]
pub struct CanvasState {
    pub model: Model, // our program's data model -> can be accessed from the root state
    radius: f32,
}

impl Default for CanvasState {
    fn default() -> CanvasState {
        CanvasState { radius: 5.0, model: Model::default() }
    }
}

impl CanvasState {

}

// Then, we implement the `Program` trait
impl canvas::Program<Message> for CanvasState {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let mut frame = Frame::new(bounds.size());

        // TODO: get an immutable reference from our state here....
        let note_blocks: &Vec<NoteBlock> = &(self.model).floating_note_blocks;
        for note_block in note_blocks {
            match note_block.location {
                Some(pos) => {
                    // TODO: make a simple 2d camera for redering
                    frame.fill_rectangle(pos, NoteBlock::size(), Light1);
                }
                None => {
                    println!("TODO: draw this in reference to another noteblock?");
                }
            }

        }

        frame.fill_rectangle(Point::new(0.0, 0.0), frame.size(), Dark2);

        // We create a `Path` representing a simple circle & fill it
        let circle = Path::circle(frame.center(), self.radius);
        frame.fill(&circle, DarkFull);

        // Finally, we produce the geometry
        vec![frame.into_geometry()]
    }

    fn update(&mut self, event: canvas::Event, _bounds: Rectangle<f32>, _cursor: Cursor) -> (Status, Option<Message>) {
        match event {
            Keyboard(event) => match event {
                keyboard::Event::KeyPressed{key_code, modifiers} => {
                    println!("KeyPressed Event -> {:?} {:?}", key_code, modifiers);
                    return match key_code {
                        KeyCode::N => (Status::Captured, Some(Message::AddNoteBlock(None))),
                        _ => (Status::Captured, None),
                    };
                },
                _ => (Status::Ignored, None)
            },
            Mouse(_) => (Status::Ignored, None)
        }
    }
}