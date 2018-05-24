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

pub fn middle_of_ohlc(ohlc: OHLC) -> f64 {
	((ohlc.h - ohlc.l) / 2.) + ohlc.l
}
