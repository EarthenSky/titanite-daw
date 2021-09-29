
/*
// Ahhhhhhhhhhhhhhh why is this already so complicated
fn sin_wave(sample_rate: u32, seconds: u32, ) -> Vec<i16> {
    for t in (0 .. seconds * sample_rate).map(|x| x as f32 / 44100.0) {
        let sample = (t * 440.0 * 2.0 * PI).sin(); // our sample is [0, 1]
        let amplitude = i16::MAX as f32;
        
        let intSample = (sample * amplitude) as i16; // we scale our sample to 
        writer.write_sample(intSample).unwrap();
    }
}
*/