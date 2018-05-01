use api::RendererExtension;
use data::OHLC;
use model::ChartBuffer;

#[derive(Debug)]
pub struct OHLCCandles {
	up_colour: u32,
	down_colour: u32,
}

impl OHLCCandles {
	pub fn new(up_colour: u32, down_colour: u32) -> OHLCCandles {
		OHLCCandles { up_colour, down_colour }
	}
}

impl RendererExtension for OHLCCandles {
	fn apply(&self, buffer: &mut ChartBuffer, data: &[OHLC]) {
		let period = buffer.timeframe / data.len() as i64;

		for i in 0..data.len() {
			let ohlc = data[i];

			let colour = if ohlc.o > ohlc.c { self.down_colour } else { self.up_colour };

			{
				let p1 = buffer.data_to_coords(ohlc.o, period * i as i64);
				let p2 = buffer.data_to_coords(ohlc.c, period * (i as i64 + 1) - 1);

				buffer.rect_point(p1, p2, colour);
			}

			{
				let time = period * i as i64 + (period / 2);
				let p1 = buffer.data_to_coords(ohlc.h, time - (period / 8));
				let p2 = buffer.data_to_coords(ohlc.l, time + (period / 8));

				buffer.rect_point(p1, p2, colour);
			}
		}
	}

	fn name(&self) -> String {
		"OHLC_Candles()".to_string()
	}
}
