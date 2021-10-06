// do: defines the model and other important data structures that make up this model.

use iced::{Point, Size};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Model {
    export_settings: ExportSettings, // TODO: functions to update settings
    playback_settings: PlaybackSettings,
    song_settings: SongSettings,
    pub note_racks: Vec<NoteRack>,
    pub floating_note_blocks: Vec<NoteBlock>,
}
impl Default for Model {
    fn default() -> Model {
        Model {
            export_settings: ExportSettings::default(),
            playback_settings: PlaybackSettings::default(),
            song_settings: SongSettings::default(),
            note_racks: Vec::new(),
            floating_note_blocks: Vec::new(),
        }
    }
}
impl Model {
    pub fn create_note_block(&mut self, parent_rack: Option<u32>) -> bool {
        match parent_rack {
            Some(_rack_id) => println!("TODO: this"),
            None => {
                let note_block = NoteBlock::new(String::from("A"), Some(Point::new(0.0, 0.0))); // TODO: make a generator to give incermenetal letters.
                self.floating_note_blocks.push(note_block);
            }
        } 
        true
    }
}

// --------------------------------------------------------------------- //

#[derive(Debug)]
struct PlaybackSettings {
    bitrate: u32,
    sample_rate: u32,
    sample_size: u32, // how big of a chunk should be rendered at a time, then played back.
}
impl PlaybackSettings {
    const fn default() -> PlaybackSettings {
        PlaybackSettings {
            bitrate: 16,
            sample_rate: 44100,
            sample_size: 400, // I don't know what a good length for this is...
        }
    }
}

#[derive(Debug)]
struct ExportSettings {
    bitrate: u32,
    sample_rate: u32,
}
impl ExportSettings {
    const fn default() -> ExportSettings {
        ExportSettings {
            bitrate: 16,
            sample_rate: 44100,
        }
    }
}

#[derive(Debug)]
struct SongSettings {
    title: String,
}
impl SongSettings {
    const fn default() -> SongSettings {
        SongSettings {
            title: String::new(),
        }
    }
}

// --------------------------------------------------------------------- //

#[derive(Debug)]
pub struct NoteRack {
    name: String,
    blocks: Vec<NoteBlock>,
}

// TODO: figure out which dataset I want to use to store this info
// - https://github.com/Amanieu/hashbrown 
// - https://crates.io/keywords/hashmap
// Requirements:
// - get the age of all notes at a certain timestep.
// OR get the starting point of all notes in a range.
#[derive(Debug)]
pub struct NoteBlock {
    pub location: Option<Point>, // Lookup 2d point vector
    pub name: String,
    pub notes: Vec<(f64, MIDINote)>,
}
impl NoteBlock {
    pub fn new(name: String, location: Option<Point>) -> NoteBlock {
        NoteBlock {
            location: location,
            name: name,
            notes: Vec::new(),
        }
    }

    pub fn size() -> Size {
        Size::new(128.0, 64.0)
    }
}

// notes aren't directly associated with the time they start?
#[derive(Debug)]
pub struct MIDINote {
    pub pitch: Note,
    pub length: f64,
    pub sensitivity: f64,
}

#[derive(Debug)]
pub enum Note {
    G9 = 127,
    // ...

    C4,
    D4,
    E5,
    F5,
    // ...
    Cn1 = 0,
    // TODO: ... 
}

// TODO: define this properly
//static note_to_hz: HashMap<Note, f32> = HashMap::new();

// --------------------------------------------------------------------- //
