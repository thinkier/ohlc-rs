extern crate ohlc;
extern crate serde_json;
extern crate argh;

use argh::FromArgs;
use ohlc::*;
use ohlc::model::rex::{BollingerBands, RSI, EMA, DEMA, MACD};
use std::fs;

#[derive(FromArgs)]
/// frontend for generating charts using ohlc-rs
struct CliOptions {
	/// source data file to feed into the chart generator
	#[argh(positional)]
	input: String,
	/// where the image is (over)written to
	#[argh(option, default = "default_output()", short = 'o')]
	output: String,

	/// include Bollinger Bands (20, 2)
	#[argh(switch)]
	bb: bool,
	/// include Relative Strength Index (10)
	#[argh(switch)]
	rsi: bool,
	/// include Exponential Moving Averages (20, sf=0.1)
	#[argh(switch)]
	ema: bool,
	/// include Double Exponential Moving Averages (20, sf=0.1)
	#[argh(switch)]
	dema: bool,
	/// include Moving Average Convergence Divergence (20, sf=0.1)
	#[argh(switch)]
	macd: bool,
}

fn default_output() -> String {
	return "out.png".to_string();
}

fn main() {
	let options: CliOptions = argh::from_env();

	let mut ohlc: OHLCRenderOptions = OHLCRenderOptions::new();

	ohlc.title("ohlc-rs demo", 0xFFFFFFFF);
	ohlc.background_colour(0x444444FF);
	ohlc.line(0xEEEEEEFF, 500.0, 24);

	if options.bb {
		ohlc.add_extension(BollingerBands::new(20, 2, 0x00AAAAFF));
	}
	if options.rsi {
		ohlc.add_extension(RSI::new(0xFFFFFFFF, 0xFF7F00FF, 0xFF0000FF, 0x00FF00FF));
	}
	if options.ema {
		ohlc.add_extension(EMA::new(20, 0.1, 0xEE00EE9F));
	}
	if options.dema {
		ohlc.add_extension(DEMA::new(EMA::new(20, 0.1, 0x007FFF9F)));
	}
	if options.macd {
		ohlc.add_extension(MACD::new(0x00FF00FF, 0xFF0000FF, 0x7F9F00FF, 0xFFFFFFFF, 0.1));
	}

	ohlc.render(get_data(&options.input), |p| fs::rename(p, &options.output)
		.map_err(|err| format!("{:?}", err))).unwrap().unwrap();
}

fn get_data(path: &str) -> Vec<OHLC> {
	return serde_json::from_reader(fs::OpenOptions::new()
		.read(true)
		.open(path)
		.unwrap()
	).unwrap();
}