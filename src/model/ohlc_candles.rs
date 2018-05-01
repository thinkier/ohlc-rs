#[derive(Debug)]
pub struct OHLCCandles {
	periods: usize,
	standard_deviations: usize,
	line_colour: u32,
}

impl OHLCCandles {
	pub fn new(periods: usize, standard_deviations: usize, line_colour: u32) -> OHLCCandles {
		OHLCCandles { periods, standard_deviations, line_colour }
	}
}

