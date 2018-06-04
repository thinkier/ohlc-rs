pub use buffer::*;
pub use data::OHLC;
pub use model::rex::volume::Volume;
pub use OHLCRenderOptions;
pub use self::basic_indicative_lines::BasicIndicativeLines;
pub use self::bollinger_bands::BollingerBands;
pub use self::dema::DEMA;
pub use self::ema::EMA;
pub use self::grid_lines::GridLines;
pub use self::macd::MACD;
pub use self::no_extension::NoExtension;
pub use self::ohlc_candles::OHLCCandles;
pub use self::rsi::RSI;
use std::fmt::Debug;


pub trait RendererExtension: Debug {
	fn apply(&self, _buffer: &mut ChartBuffer, _data: &[OHLC]);

	fn lore_colour(&self) -> Option<u32>;

	fn name(&self) -> String;
}

pub mod basic_indicative_lines;
pub mod bollinger_bands;
pub mod dema;
pub mod ema;
pub mod grid_lines;
pub mod macd;
pub mod no_extension;
pub mod ohlc_candles;
pub mod rsi;
pub mod volume;
#[cfg(test)]
pub mod test_fill;
#[cfg(test)]
pub mod test_line;
#[cfg(test)]
pub mod test_text;

