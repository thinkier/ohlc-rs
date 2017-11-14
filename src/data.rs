#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct OHLC {
	o: f64,
	h: f64,
	l: f64,
	c: f64,
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