
struct Model {
    export_settings: ExportSettings,
    playback_settings: PlaybackSettings,
    settings: SongSettings,
    note_racks: Vec<NoteRack>,
}

// --------------------------------------------------------------------- //

struct PlaybackSettings {
    bitrate: u32,
    sample_rate: u32,
    sample_size: u32, // how big of a chunk should be rendered at a time, then played back.
}

struct ExportSettings {
    bitrate: u32,
    sample_rate: u32,
}

struct SongSettings {
    title: String,
}

// --------------------------------------------------------------------- //

struct NoteRack {
    blocks: Vec<NoteBlock>,
}

// TODO: figure out which dataset I want to use to store this info
// - https://github.com/Amanieu/hashbrown 
// - https://crates.io/keywords/hashmap
// Requirements:
// - get the age of all notes at a certain timestep.
// OR get the starting point of all notes in a range.
struct NoteBlock {
    notes: Vec<(f64, Note)>,
}

// notes aren't directly associated with the time they start?
struct Note {
    pitch: NotePitch,
    length: f64,
    sensitivity: f64,
}

enum NotePitch {
    C4,
    D4,
    E5,
    F5,
    // TODO: ... 
}

// --------------------------------------------------------------------- //
