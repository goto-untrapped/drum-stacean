use opengl_graphics::{GlGraphics, Texture as GlTexture};
use piston_window::{Image, DrawState, Context};

use crate::Sound;
#[derive(Debug)]
pub struct EffectFont {
    pub x: f64,
    pub y: f64,
    pub status: Sound,
}

pub struct EffectFonts {
    pub fonts: Vec<EffectFont>,
}

impl EffectFonts {
    pub fn new() -> EffectFonts {
        let effect_fonts = EffectFonts {
            fonts: Vec::<EffectFont>::new(),
        };
        effect_fonts
    }

    pub fn add(&mut self, sound_type: &Sound) {
        // when key press, add pressed sound to vec
        match sound_type {
            Sound::BassDrum => {
                let effect_font = EffectFont {
                    x: 360.0,
                    y: 340.0,
                    status: Sound::BassDrum,
                };
                // push to vec
                self.fonts.push(effect_font);
            },
            Sound::SnareDrum => {
                let effect_font = EffectFont {
                    x: 300.0,
                    y: 300.0,
                    status: Sound::SnareDrum,
                };
                // push to vec
                self.fonts.push(effect_font);
            },
            _ => (),
        }
    }

    pub fn render(&mut self, c: &Context, gl: &mut GlGraphics) {}
}