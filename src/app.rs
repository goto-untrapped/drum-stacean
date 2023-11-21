use std::path::{PathBuf, Path};
use piston_window::*;
use crate::{effect_font::EffectFonts, Sound};

use opengl_graphics::{GlGraphics, Texture as GlTexture};

pub struct App {
    effect_fonts: EffectFonts,
    pic_drum: Option<GlTexture>,
    effect_don: Option<GlTexture>, // Bass Drum
    effect_tan: Option<GlTexture>, // Snare Drum
    effect_tsu: Option<GlTexture>, // Hihat Drum
    effect_pon: Option<GlTexture>, // High Tom
    effect_bon: Option<GlTexture>, // Medium Tom
    effect_poko: Option<GlTexture>, // Fllor Tom
    effect_tuuun: Option<GlTexture>, // Ride Cymbal
    effect_shaan: Option<GlTexture>, // Crash Cymbal
}

impl App {
    pub fn new() -> App {
        App {
            effect_fonts: EffectFonts::new(),
            pic_drum: None,
            effect_don: None,
            effect_tan: None,
            effect_tsu: None,
            effect_pon: None,
            effect_bon: None,
            effect_poko: None,
            effect_tuuun: None,
            effect_shaan: None,
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
        // Bass Drum
        self.effect_don = self.load_effect_image(asset_root.clone(), "don.png");
        // Snare Drum
        self.effect_tan = self.load_effect_image(asset_root.clone(), "tan.png");
        // Hihat Drum

        // High Tom

        // Medium Tom

        // Fllor Tom

        // Ride Cymbal

        // Crash Cymbal

    }

    fn load_effect_image(&mut self, root: PathBuf, image_path: &str) -> Option<GlTexture> {
        let mut effect_path = root.clone();
        let texture_settings = TextureSettings::new();
        effect_path.push(Path::new(image_path));
        Some(GlTexture::from_path(&effect_path, &texture_settings).unwrap())
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
                    Sound::SnareDrum => {
                        Image::new()
                        .draw(self.effect_tan.iter().next().unwrap(),
                            &DrawState::default(),
                            c.trans(font.x,font.y).transform,
                            gl);
                    },
                    _ => {}
                }
            }
        });

    }

    pub fn update(&mut self, args: &UpdateArgs) {}

    pub fn add(&mut self, sound_type: &Sound) {
        self.effect_fonts.add(sound_type);
        println!("{:?}", &self.effect_fonts.fonts);
    }

    pub fn clear_effect_font(&mut self) {
        // clear font
        self.effect_fonts = EffectFonts::new();
    }

}