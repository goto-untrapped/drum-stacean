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
                    x: 370.0,
                    y: 350.0,
                    status: Sound::BassDrum,
                };
                // push to vec
                self.fonts.push(effect_font);
            },
            Sound::SnareDrum => {
                let effect_font = EffectFont {
                    x: 210.0,
                    y: 380.0,
                    status: Sound::SnareDrum,
                };
                // push to vec
                self.fonts.push(effect_font);
            },
            Sound::HiHat => {
                let effect_font = EffectFont {
                    x: 80.0,
                    y: 200.0,
                    status: Sound::HiHat,
                };
                // push to vec
                self.fonts.push(effect_font);
            },
            Sound::FloorTom => {
                let effect_font = EffectFont {
                    x: 600.0,
                    y: 230.0,
                    status: Sound::FloorTom,
                };
                // push to vec
                self.fonts.push(effect_font);
            },
            Sound::HighTom => {
                let effect_font = EffectFont {
                    x: 180.0,
                    y: 150.0,
                    status: Sound::HighTom,
                };
                // push to vec
                self.fonts.push(effect_font);
            },
            Sound::MediumTom => {
                let effect_font = EffectFont {
                    x: 560.0,
                    y: 150.0,
                    status: Sound::MediumTom,
                };
                // push to vec
                self.fonts.push(effect_font);
            },
            Sound::RideCymbal => {
                let effect_font = EffectFont {
                    x: 600.0,
                    y: 30.0,
                    status: Sound::RideCymbal,
                };
                // push to vec
                self.fonts.push(effect_font);
            },
            Sound::CrashCymbal => {
                let effect_font = EffectFont {
                    x: 50.0,
                    y: 100.0,
                    status: Sound::CrashCymbal,
                };
                // push to vec
                self.fonts.push(effect_font);
            },
            
        }
    }

    pub fn render(&mut self, c: &Context, gl: &mut GlGraphics) {}
}