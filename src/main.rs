use piston_window::*;
mod app;
mod effect_font;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Music {}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum Sound {
    BassDrum,
    SnareDrum,
    HiHat,
    FloorTom,
    HighTom,
    MediumTom,
    RideCymbal,
    CrashCymbal
}

fn main() {
    use opengl_graphics::GlGraphics;
    let (width, height) = (895, 700);
    // initialize window
    let mut window: PistonWindow = 
        WindowSettings::new("drum_stacean" , [width, height])
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    let mut app = app::App::new();

    // load image
    app.load();

    let mut gl = GlGraphics::new(OpenGL::V3_2);

    music::start::<Music, Sound, _>(16, || {

        while let Some(e) = window.next() {
            // update related to graphic
            if let Some(ref args) = e.render_args() {
                app.render(args, &mut gl);
            }

            // update related to variable
            if let Some(ref args) = e.update_args() {
                app.update(args);
            }

            // keyboard input
            if let Some(ref args) = e.press_args() {
                use piston_window::Button::Keyboard;
                // app.key_press(args);

                // Bass Drum
                if *args == Keyboard(Key::Space) {
                    println!("I'm BassDrum!");
                    make_sound(Sound::BassDrum, "bin/assets/bass_drum.mp3");
                    app.add(&Sound::BassDrum);
                }
                // Snare
                if *args == Keyboard(Key::F) || *args == Keyboard(Key::J) {
                    println!("I'm Snare!");
                    make_sound(Sound::SnareDrum, "bin/assets/snare_drum.mp3");
                }
                // HiHat Open
                if *args == Keyboard(Key::D) || *args == Keyboard(Key::K) {
                    println!("I'm Hihat!");
                    make_sound(Sound::HiHat, "bin/assets/hi_hat.mp3");
                }
                // HiHat Close
                // High Tom right side one
                if *args == Keyboard(Key::S) {
                    println!("I'm High Tom!");
                    make_sound(Sound::HighTom, "bin/assets/high_tom.mp3");
                }
                // Medium Tom left side one
                if *args == Keyboard(Key::L) {
                    println!("I'm Medium Tom!");
                    make_sound(Sound::MediumTom, "bin/assets/medium_tom.mp3");
                }
                // Floor Tom
                if *args == Keyboard(Key::Semicolon) {
                    println!("I'm Floor Tom!");
                    make_sound(Sound::FloorTom, "bin/assets/floor_tom.mp3");
                }
                // Ride Cymbal
                if *args == Keyboard(Key::Unknown) {
                    println!("I'm Ride Cymbal!");
                    make_sound(Sound::RideCymbal, "bin/assets/ride_cymbal.mp3");
                }
                // Crash Cymbal left up side one
                if *args == Keyboard(Key::A) {
                    println!("I'm Crash Cymbal!");
                    make_sound(Sound::CrashCymbal, "bin/assets/crash_cymbal.mp3");
                }
            }
        }
    });

    fn make_sound(sound_type: Sound, file_name: &str) {
        music::bind_sound_file(sound_type, file_name);
        music::set_volume(music::MAX_VOLUME);
        music::play_sound(&sound_type, music::Repeat::Times(0), music::MAX_VOLUME);
    }

}
