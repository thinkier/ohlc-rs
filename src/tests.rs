extern crate env_logger;
extern crate serde_json;

use model::bollinger_bands::*;
use model::no_extension::NoExtension;
use model::test_fill::TestFill;
use model::test_line::TestLine;
use super::*;

#[test]
fn render_options_modification() {
	assert_eq!(
		OHLCRenderOptions {
			title: String::new(),
			title_colour: 0,
			background_colour: 0xFEFEFEFF,
			current_value_colour: 0x69696968,
			time_units: 3600,
			line_colour: 6969,
			price_line_interval: 69.69,
			time_line_interval: 69,
			down_colour: 0x69696969,
			up_colour: 0x69696970,
			render_extensions: NoExtension {},
		},
		OHLCRenderOptions::new(NoExtension {})
			.indicator_colours(0x69696968, 0x69696969, 0x69696970)
			.background_colour(0xFEFEFEFF)
			.line(6969, 69.69, 69)
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
	let _ = env_logger::try_init();

	let data: Vec<OHLC> = self::serde_json::from_str(include_str!("../sample_data.json")).unwrap();
	let options = OHLCRenderOptions::new(NoExtension {})
		.title("BTCUSD | ohlc-rs", 0x007F7FFF)
		.line(0xCCCCCCFF, 200., 24)
		.background_colour(0x36393EFF);

	options.render_and_save(
		data.clone(),
		&Path::new("test-draw-sample-data.png"),
	).unwrap();
}

#[test]
fn render_draw_sample_data_with_bb() {
	let _ = env_logger::try_init();

	let data: Vec<OHLC> = self::serde_json::from_str(include_str!("../sample_data.json")).unwrap();
	let bb = BollingerBands::new(20, 2, 0x0000FF7F);
	{
		let options = OHLCRenderOptions::new(bb)
			.title("BTCUSD | ohlc-rs", 0x007F7FFF)
			.line(0xCCCCCCFF, 200., 24)
			.background_colour(0x36393EFF);

		options.render_and_save(
			data.clone(),
			&Path::new("test-draw-sample-data+bb.png"),
		).unwrap();
	}
}

#[test]
fn render_draw_sample_data_with_test_text() {
	let _ = env_logger::try_init();

	let data: Vec<OHLC> = self::serde_json::from_str(include_str!("../sample_data.json")).unwrap();
	let tt = test_text::TestText {};
	{
		let options = OHLCRenderOptions::new(tt)
			.title("BTCUSD | ohlc-rs", 0x007F7FFF)
			.line(0xCCCCCCFF, 200., 24)
			.background_colour(0x36393EFF);

		options.render_and_save(
			data.clone(),
			&Path::new("test-draw-sample-data+test-text.png"),
		).unwrap();
	}
}

#[test]
fn render_draw_sample_data_with_test_fill() {
	let _ = env_logger::try_init();

	let data: Vec<OHLC> = self::serde_json::from_str(include_str!("../sample_data.json")).unwrap();
	let tf = TestFill { colour: 0xFFFF00FF };
	{
		let options = OHLCRenderOptions::new(tf)
			.title("BTCUSD | ohlc-rs", 0x007F7FFF)
			.line(0xCCCCCCFF, 200., 24)
			.background_colour(0x36393EFF);

		options.render_and_save(
			data.clone(),
			&Path::new("test-draw-sample-data+test-fill.png"),
		).unwrap();
	}
}

#[test]
fn render_draw_sample_data_with_test_fill_with_alpha() {
	let _ = env_logger::try_init();

	let data: Vec<OHLC> = self::serde_json::from_str(include_str!("../sample_data.json")).unwrap();
	let tf = TestFill { colour: 0xFFFF007F };
	{
		let options = OHLCRenderOptions::new(tf)
			.title("BTCUSD | ohlc-rs", 0x007F7FFF)
			.line(0xCCCCCCFF, 200., 24)
			.background_colour(0x36393EFF);

		let _ = options.render_and_save(
			data.clone(),
			&Path::new("test-draw-sample-data+test-fill-with-alpha.png"),
		);
	}
}

#[test]
fn render_draw_sample_data_with_test_line() {
	let _ = env_logger::try_init();

	let data: Vec<OHLC> = self::serde_json::from_str(include_str!("../sample_data.json")).unwrap();
	let ttl = TestLine {};
	{
		let options = OHLCRenderOptions::new(ttl)
			.title("BTCUSD | ohlc-rs", 0x007F7FFF)
			.line(0xCCCCCCFF, 200., 24)
			.background_colour(0x36393EFF);

		options.render_and_save(
			data.clone(),
			&Path::new("test-draw-sample-data+test-line.png"),
		).unwrap();
	}
}
