use std::path::{PathBuf, Path};
use piston_window::*;
use crate::{effect_font::EffectFonts, Sound};

use opengl_graphics::{GlGraphics, Texture as GlTexture};

pub struct App {
    effect_fonts: EffectFonts,
    pic_drum: Option<GlTexture>,
    effect_don: Option<GlTexture>,
}

impl App {
    pub fn new() -> App {
        App {
            effect_fonts: EffectFonts::new(),
            pic_drum: None,
            effect_don: None,
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

        // Effect Fonts
        let mut effect_don_path = asset_root.clone();
        effect_don_path.push(Path::new("don.png"));
        self.effect_don = Some(GlTexture::from_path(&effect_don_path, &texture_settings).unwrap());
    }

    pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        // get rendering area
        let area = args.window_size;
        let c = &Context::new_abs(area[0], area[1]);

        // draw object
        gl.draw(args.viewport(), |_, gl| {

            clear([1.0, 0.0, 0.0, 0.0], gl);
            Image::new()
                .draw(self.pic_drum.iter().next().unwrap(),
                    &DrawState::default(),
                    c.trans(0.0, 0.0).transform,
                    gl);

            // draw effect sound
            self.effect_fonts.render(c, gl);

            // render fonts in vec
            for font in self.effect_fonts.fonts.iter_mut() {
                match font.status {
                    Sound::BassDrum => {
                        Image::new()
                        .draw(self.effect_don.iter().next().unwrap(),
                            &DrawState::default(),
                            c.trans(font.x,font.y).transform,
                            gl);
                    },
                    _ => {}
                }
            }
        });

    }

    pub fn update(&mut self, args: &UpdateArgs) {
        // delete effect font
        // self.effect_fonts = EffectFonts::new();
    }

    pub fn key_press(&mut self, args: &Button) {
        use piston_window::Button::Keyboard;

        // key Space
        if *args == Keyboard(Key::Space) {
            println!("Clicked Space!");
        }
    }

    pub fn add(&mut self, sound_type: &Sound) {
        self.effect_fonts.add(sound_type);
        println!("{:?}", &self.effect_fonts.fonts);
    }

    pub fn clear_effect_font(&mut self) {
        // clear fonts
        // self.effect_fonts.fonts.retain(|font| font.status != *sound_type);
        // if let Some(index) = self.effect_fonts.fonts.iter().position(|font| font.status == *sound_type) {
        //     self.effect_fonts.fonts.remove(index);
        // }
        // println!("{:?}", &self.effect_fonts.fonts);
        self.effect_fonts = EffectFonts::new();
    }

}