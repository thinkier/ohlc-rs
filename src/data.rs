pub trait Candle {
	fn open(&self) -> f64;
	fn high(&self) -> f64;
	fn low(&self) -> f64;
	fn close(&self) -> f64;
	fn buy_volume(&self) -> Option<f64>;
	fn sell_volume(&self) -> Option<f64> {
		self.buy_volume().map(|buy| self.total_volume() - buy)
	}
	fn total_volume(&self) -> f64;
}