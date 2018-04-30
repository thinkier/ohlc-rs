extern crate env_logger;
extern crate serde_json;

use super::*;

#[test]
fn render_options_modification() {
	assert_eq!(
		OHLCRenderOptions {
			title: String::new(),
			title_colour: 0,
			background_colour: 0xFEFEFEFF,
			current_value_colour: 0x69696968,
			value_prefix: String::new(),
			value_suffix: String::new(),
			time_units: 3600,
			h_axis_options: AxisOptions::new(),
			v_axis_options: AxisOptions::new(),
			down_colour: 0x69696969,
			up_colour: 0x69696970,
			render_extensions: vec![],
		},
		OHLCRenderOptions::new()
			.indicator_colours(0x69696968, 0x69696969, 0x69696970)
			.background_colour(0xFEFEFEFF)
	);
}

#[test]
fn axis_options_modification() {
	assert_eq!(
		AxisOptions {
			title: "I'm a meme".to_string(),
			title_colour: 69,
			line_colour: 70,
			line_frequency: 71.,
			label_colour: 72,
			label_frequency: 73.,
		},
		AxisOptions::new()
			.title("I'm a meme", 69)
			.line(70, 71.)
			.label(72, 73.)
	);
}

/*
#[test]
fn render_repetition() {
	let _ = OHLCRenderOptions::new()
		.render_and_save(
			vec![OHLC { o: 2.0, h: 4.0, l: 0.0, c: 1.0 }; 168],
			&Path::new("test-repetition.png")
		);
}

#[test]
fn render_draw_v_axis_lines() {
	let _ = OHLCRenderOptions::new()
		.v_axis(|va| va
			.line_colour(0x000000FF)
			.line_frequency(5.)
		)
		.render_and_save(
			vec![OHLC { o: 2.0, h: 12.0, l: 0.0, c: 6.0 }; 168],
			&Path::new("test-draw-lines-vaxis.png")
		);
}

#[test]
fn render_up_down() {
	let _ = OHLCRenderOptions::new()
		.render_and_save(
			vec![
				OHLC { o: 1.0, h: 4.0, l: 0.0, c: 2.0 },
				OHLC { o: 2.0, h: 4.0, l: 0.0, c: 1.0 }
			],
			&Path::new("test-up-down.png")
		);
}

#[test]
fn render_temp_copy() {
	let _ = OHLCRenderOptions::new()
		.render(
			vec![OHLC { o: 2.0, h: 4.0, l: 0.0, c: 1.0 }; 3],
			|path| if let Err(err) = fs::copy(path, &Path::new("test-temp-copy.png")) {
				Err(format!("File copy error: {:?}", err))
			} else {
				Ok(())
			});
}
*/

#[test]
fn render_draw_sample_data() {
	env_logger::init();

	let data: Vec<OHLC> = self::serde_json::from_str(include_str!("../sample_data.json")).unwrap();
	let options = OHLCRenderOptions::new()
		.title("BTCUSD | ohlc-rs", 0x007F7FFF)
		.v_axis(|va| va
			.line(0xCCCCCCFF, 200.)
			.label(0x222222FF, 200.)
		)
		.h_axis(|va| va
			.line(0xD2D2D2FF, 24.)
			.label(0x222222FF, 24.)
		)
		.background_colour(0x36393EFF)
		.value_strings("$", "");

	let _ = options.render_and_save(
		data.clone(),
		&Path::new("test-draw-sample-data.png"),
	);
	let _ = options.render_and_save(
		data,
		&Path::new("test-draw-sample-data.bmp"),
	);
}
