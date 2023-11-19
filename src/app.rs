use std::path::{PathBuf, Path};
use piston_window::*;

use opengl_graphics::{GlGraphics, Texture as GlTexture};

pub struct App {
    pic_drum: Option<GlTexture>,
}

impl App {
    pub fn new() -> App {
        App {
            pic_drum: None,
        }
    }

    pub fn load(&mut self) {
        // load assets folder path
        let mut asset_root = PathBuf::new();
        asset_root.push("bin/assets".to_string());

        // load image path
        let mut logo_path = asset_root.clone();
        logo_path.push(Path::new("drumset.png"));

        let texture_settings = TextureSettings::new();
        // load image
        self.pic_drum = Some(GlTexture::from_path(&logo_path, &texture_settings).unwrap());
    }

    pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        // get rendering area
        let area = args.window_size;
        let c = &Context::new_abs(area[0], area[1]);

        // draw object
        gl.draw(args.viewport(), |_, gl| {
            // draw drumset
            Image::new()
                .draw(self.pic_drum.iter().next().unwrap(),
                    &DrawState::default(),
                    c.trans(0.0, 0.0).transform,
                    gl);            
        });

    }

    pub fn update(&mut self, args: &UpdateArgs) {
        
    }

    pub fn key_press(&mut self, args: &Button) {
        use piston_window::Button::Keyboard;

        // key Space
        if *args == Keyboard(Key::Space) {
            println!("Clicked Space!");
        }
    }

    
}