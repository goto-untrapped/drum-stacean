use opengl_graphics::{GlGraphics, Texture as GlTexture};
use piston_window::{Image, DrawState, Context};

use crate::Sound;

struct EffectFont {
    x: i32,
    y: i32,
    status: Sound,
    effect: Option<GlTexture>,
}

pub struct EffectFonts {
    fonts: Vec<EffectFont>,
}

impl EffectFonts {
    pub fn new() -> EffectFonts {
        let effect_fonts = EffectFonts {
            fonts: Vec::<EffectFont>::new(),
        };
        effect_fonts
    }

    pub fn add(&mut self, sound_type: &Sound, effect: Option<GlTexture>) {
        // when key press, add pressed sound to vec
        match sound_type {
            Sound::BassDrum => {
                let effect_font = EffectFont {
                    x: 200,
                    y: 100,
                    status: Sound::BassDrum,
                    effect: effect,
                };
                // push to vec
                self.fonts.push(effect_font);
            },
            _ => (),
        }
    }

    pub fn render(&mut self, c: &Context, gl: &mut GlGraphics) {
        // render fonts in vec
        for font in self.fonts.iter() {
            Image::new()
            .draw(font.effect.iter().next().unwrap(),
                &DrawState::default(),
                c.transform,
                gl);
        }
    }
}