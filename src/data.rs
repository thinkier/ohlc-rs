pub trait Candle {
	fn open(&self) -> f64;
	fn high(&self) -> f64;
	fn low(&self) -> f64;
	fn close(&self) -> f64;
	fn buy_volume(&self) -> f64;
	fn sell_volume(&self) -> f64 {
		self.total_volume() - self.buy_volume()
	}
	fn total_volume(&self) -> f64;
}