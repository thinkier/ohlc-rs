use model::*;
use model::buffer::ChartBuffer;

#[derive(Clone, Debug)]
pub struct EMA {
	pub(crate) periods: usize,
	pub(crate) smoothing_factor: f64,
	pub(crate) colour: u32,
}

impl EMA {
	pub fn new(periods: usize, smoothing_factor: f64, colour: u32) -> EMA {
		EMA { periods, smoothing_factor, colour }
	}
}

impl RendererExtension for EMA {
	fn apply(&self, buffer: &mut ChartBuffer, data: &[OHLC]) {
		let tf = buffer.timeframe;
		let len = data.len();
		let ema = ema(&self, &median_list(data));

		for p in self.periods + 1..len {
			let p1 = buffer.data_to_coords(ema[p - 1], (tf as f64 * ((p - 1) as f64 / len as f64)) as i64);
			let p2 = buffer.data_to_coords(ema[p], (tf as f64 * (p as f64 / len as f64)) as i64);

			buffer.line(p1, p2, self.colour);
		}
	}

	fn lore_colour(&self) -> Option<u32> {
		Some(self.colour)
	}

	fn name(&self) -> String {
		format!("EMA({}, sf={})", self.periods, self.smoothing_factor)
	}
}

pub fn ema(ema: &EMA, data: &[f64]) -> Vec<f64> {
	let mut buf = vec![];

	for point in 0..data.len() {
		let mut numerator = 0.;
		let mut denominator = 0.;
		for i in if point > ema.periods { point - ema.periods } else { 0 }..point + 1 {
			let exponent = (point + 1) - i;
			let weight = (1. - ema.smoothing_factor).powf(exponent as f64);

			numerator += data[i] * weight;
			denominator += weight;
		}

		buf.push(numerator / denominator);
	}

	return buf;
}

pub fn median_of_ohlc(ohlc: OHLC) -> f64 {
	((ohlc.h - ohlc.l) / 2.) + ohlc.l
}

pub fn median_list(list: &[OHLC]) -> Vec<f64> {
	let mut buf = vec![];

	for ohlc in list {
		buf.push(median_of_ohlc(*ohlc));
	}

	buf
}
