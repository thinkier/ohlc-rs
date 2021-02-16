use std::marker::PhantomData;

use model::*;

#[derive(Clone, Debug)]
pub struct OHLCCandles<C> {
	_c: PhantomData<C>,
	up_colour: u32,
	down_colour: u32,
}

impl<C> OHLCCandles<C> {
	pub fn new(up_colour: u32, down_colour: u32) -> OHLCCandles<C> {
		OHLCCandles { _c: PhantomData, up_colour, down_colour }
	}
}

impl<C: Candle> RendererExtension for OHLCCandles<C> {
	type Candle = C;

	fn apply(&self, buffer: &mut ChartBuffer, data: &[C]) {
		let period = buffer.timeframe / data.len() as i64;
		let period_addition = 4. * period as f64 / 5.;

		for i in 0..data.len() {
			let candle = &data[i];

			let open = candle.open();
			let close = candle.close();

			let colour = if open > close { self.down_colour } else { self.up_colour };

			// Main big block
			{
				let p1 = buffer.data_to_coords(open, period * i as i64);
				let p2 = buffer.data_to_coords(close, ((period * (i as i64)) as f64 + period_addition) as i64);

				buffer.rect_point(p1, p2, colour);
			}

			// Sticks
			{
				let time = period * i as i64 + (period_addition / 2.) as i64;
				let p1 = buffer.data_to_coords(candle.high(), time - (period_addition / 12.).ceil() as i64);
				let p2 = buffer.data_to_coords(candle.low(), time + (period_addition / 12.).floor() as i64);

				buffer.rect_point(p1, p2, colour);
			}
		}
	}

	fn lore_colour(&self) -> Option<u32> {
		None
	}

	fn name(&self) -> String {
		"OHLC_Candles()".to_string()
	}
}
