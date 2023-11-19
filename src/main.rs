use piston_window::*;
mod app;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Music {}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Sound {
    Ding,
}

fn main() {
    use opengl_graphics::GlGraphics;
    let (width, height) = (895, 700);
    // initialize window
    let mut window: PistonWindow = 
        WindowSettings::new("drum-stacean" , [width, height])
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    // test music library
    // music::start::<Music, Sound, _>(16, || {
    //     music::bind_sound_file(Sound::Ding, "bin/assets/ding.mp3");
 
    //     music::set_volume(music::MAX_VOLUME);
    //     // music::play_music(&Music::Piano, music::Repeat::Forever);
    //     music::play_sound(&Sound::Ding, music::Repeat::Times(1), music::MAX_VOLUME);
    //     while let Some(e) = window.next() {
    //         // window.draw_2d(&e, |_c, g, _device| {
    //         //     clear([1.0; 4], g);
    //         // });
    //     }
    // });

    let mut app = app::App::new();

    // load assets
    app.load();

    let mut gl = GlGraphics::new(OpenGL::V3_2);

    // test music library
    music::start::<Music, Sound, _>(16, || {

        while let Some(e) = window.next() {
            // update related to graphic
            if let Some(ref args) = e.render_args() {
                app.render(args, &mut gl);
            }

            //update related to variable
            if let Some(ref args) = e.update_args() {
                app.update(args);
            }

            // keyboard input
            if let Some(ref args) = e.press_args() {
                use piston_window::Button::Keyboard;

                // app.key_press(args);
                if *args == Keyboard(Key::Space) {
                    make_sound(Sound::Ding, "bin/assets/ding.mp3");
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
