pub trait Candle {
	fn open(&self) -> f64;
	fn high(&self) -> f64;
	fn low(&self) -> f64;
	fn close(&self) -> f64;
	fn volume(&self) -> f64;
}