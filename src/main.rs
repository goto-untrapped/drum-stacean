use piston_window::*;

mod app;

fn main() {
    use opengl_graphics::GlGraphics;
    let (width, height) = (400, 300);
    // initialize window
    let mut window: PistonWindow = 
        WindowSettings::new("DrumStacean" , [width, height])
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    let mut app = app::App::new();

    // load assets
    app.load();

    let mut gl = GlGraphics::new(OpenGL::V3_2);

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
            app.key_press(args);
        }
    }

}
