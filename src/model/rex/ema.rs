use model::*;

#[derive(Clone, Debug)]
pub struct EMA {
	periods: usize,
	smoothing_factor: f64,
}

impl EMA {
	pub fn new(periods: usize, smoothing_factor: f64) -> EMA {
		EMA { periods, smoothing_factor }
	}
}

impl RendererExtension for EMA {
	fn apply(&self, buffer: &mut ChartBuffer, data: &[OHLC]) {
		unimplemented!()
	}

	fn name(&self) -> String {
		format!("EMA({}, {})", self.periods, self.smoothing_factor)
	}
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
