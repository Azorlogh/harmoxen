use druid::{Color, Env, Key};

pub const COLOR_0: Key<Color> = Key::new("color_0");
pub const COLOR_1: Key<Color> = Key::new("color_1");
pub const COLOR_2: Key<Color> = Key::new("color_2");
pub const COLOR_3: Key<Color> = Key::new("color_3");
pub const COLOR_4: Key<Color> = Key::new("color_4");

pub const NEUTRAL_COLOR_0: Key<Color> = Key::new("neutral_color_0");
pub const NEUTRAL_COLOR_1: Key<Color> = Key::new("neutral_color_1");
pub const NEUTRAL_COLOR_2: Key<Color> = Key::new("neutral_color_2");

pub const FEATURE_COLOR: Key<Color> = Key::new("feature_color");
pub const HIGHLIGHTED_COLOR: Key<Color> = Key::new("highlighted_color");
pub const SELECTED_COLOR: Key<Color> = Key::new("selected_color");
pub const BG_FEATURE_COLOR: Key<Color> = Key::new("bg_feature_color");

pub const BACKGROUND_0: Key<Color> = Key::new("background_0");
pub const BACKGROUND_1: Key<Color> = Key::new("background_1");
pub const BACKGROUND_2: Key<Color> = Key::new("background_2");

pub const NOTE_HEIGHT: Key<f64> = Key::new("note_height");
pub const NOTE_SCALE_KNOB: Key<f64> = Key::new("note_scale_knob");

fn color(code: usize) -> Color {
	Color::rgb8((code >> 16) as u8, (code >> 8) as u8, code as u8)
}

pub fn apply(env: &mut Env, _data: &crate::state::State) {
	env.set(NEUTRAL_COLOR_0, color(0xb4bfbf));
	env.set(NEUTRAL_COLOR_1, color(0x5d5d5d));
	env.set(NEUTRAL_COLOR_2, color(0x8c9191));

	env.set(COLOR_0, color(0x56bfbc));
	env.set(COLOR_1, color(0x356160));
	env.set(COLOR_2, color(0x370B59));
	env.set(COLOR_3, color(0x287573));
	env.set(COLOR_4, color(0x38a19e));

	env.set(FEATURE_COLOR, color(0x02BC71));
	env.set(HIGHLIGHTED_COLOR, color(0x69ffa2));
	env.set(SELECTED_COLOR, color(0xd32e52));
	env.set(BG_FEATURE_COLOR, color(0x5bbd95));

	env.set(BACKGROUND_0, color(0x252729));
	env.set(BACKGROUND_1, color(0x3e444a));
	env.set(BACKGROUND_2, color(0x2c2e30));

	env.set(druid::theme::WINDOW_BACKGROUND_COLOR, color(0x323232));

	env.set(NOTE_HEIGHT, 18.0);
	env.set(NOTE_SCALE_KNOB, 32.0);
}
