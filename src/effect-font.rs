
enum Sound {
    BassDrum,
    SnareDrum,
    HiHat,
    FloorTom,
    HighTom,
    MediumTom,
    RideCymbal,
    CrashCymbal
}

struct EffectFont {
    x: i32,
    y: i32,
    status: Sound,
}

struct EffectFonts {
    fonts: Vec<EffectFont>,
}

pub fn add() {
    // when key press, add pressed sound to vec
    // set x, y from sound type (fixed position)
}

pub fn render() {
    // render fonts in vec
}