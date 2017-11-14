#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct OHLC {
	pub o: f64,
	pub h: f64,
	pub l: f64,
	pub c: f64,
}

impl OHLC {
	pub fn new() -> OHLC {
		OHLC {
			o: 0.0,
			h: 0.0,
			l: 0.0,
			c: 0.0,
		}
	}
}