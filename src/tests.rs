extern crate env_logger;
extern crate serde_json;

use model::rex::*;
use model::rex::test_fill::TestFill;
use model::rex::test_line::TestLine;
use super::*;

#[test]
fn render_draw_sample_data() {
	let _ = env_logger::try_init();

	let data: Vec<OHLC> = self::serde_json::from_str(include_str!("../sample_data.json")).unwrap();
	let mut options = OHLCRenderOptions::new();
	options.title("BTCUSD | ohlc-rs", 0x007F7FFF)
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
		let mut options = OHLCRenderOptions::new();
		options.title("BTCUSD | ohlc-rs", 0x007F7FFF)
			.line(0xCCCCCCFF, 200., 24)
			.background_colour(0x36393EFF)
			.add_extension(bb);

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
		let mut options = OHLCRenderOptions::new();
		options.title("BTCUSD | ohlc-rs", 0x007F7FFF)
			.line(0xCCCCCCFF, 200., 24)
			.background_colour(0x36393EFF)
			.add_extension(tt);
		;

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
		let mut options = OHLCRenderOptions::new();
		options.title("BTCUSD | ohlc-rs", 0x007F7FFF)
			.line(0xCCCCCCFF, 200., 24)
			.background_colour(0x36393EFF)
			.add_extension(tf);

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
		let mut options = OHLCRenderOptions::new();
		options.title("BTCUSD | ohlc-rs", 0x007F7FFF)
			.line(0xCCCCCCFF, 200., 24)
			.background_colour(0x36393EFF)
			.add_extension(tf);

		let _ = options.render_and_save(
			data.clone(),
			&Path::new("test-draw-sample-data+test-fill-with-alpha.png"),
		);
	}
}

#[test]
fn render_draw_sample_data_with_test_rsi() {
	let _ = env_logger::try_init();

	let data: Vec<OHLC> = self::serde_json::from_str(include_str!("../sample_data.json")).unwrap();
	let rsi = RSI::new(0xCCCCCCFF, 0xFFFF007F, 0x27A819FF, 0xD33040FF);
	{
		let mut options = OHLCRenderOptions::new();
		options.title("BTCUSD | ohlc-rs", 0x007F7FFF)
			.line(0xCCCCCCFF, 200., 24)
			.background_colour(0x36393EFF)
			.add_extension(rsi);

		let _ = options.render_and_save(
			data.clone(),
			&Path::new("test-draw-sample-data+test-rsi.png"),
		);
	}
}

#[test]
fn render_draw_sample_data_with_test_line() {
	let _ = env_logger::try_init();

	let data: Vec<OHLC> = self::serde_json::from_str(include_str!("../sample_data.json")).unwrap();
	let ttl = TestLine {};
	{
		let mut options = OHLCRenderOptions::new();
		options.title("BTCUSD | ohlc-rs", 0x007F7FFF)
			.line(0xCCCCCCFF, 200., 24)
			.background_colour(0x36393EFF)
			.add_extension(ttl);

		options.render_and_save(
			data.clone(),
			&Path::new("test-draw-sample-data+test-line.png"),
		).unwrap();
	}
}
