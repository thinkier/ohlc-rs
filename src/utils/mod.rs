use super::*;

#[cfg(test)]
mod tests;

#[derive(Default)]
pub struct SetAggregate {
	pub o: f64,
	pub h: f64,
	pub l: f64,
	pub c: f64,
	pub bv: f64,
	pub v: f64,
}

impl Candle for SetAggregate {
	#[inline]
	fn open(&self) -> f64 { self.o }
	#[inline]
	fn high(&self) -> f64 { self.h }
	#[inline]
	fn low(&self) -> f64 { self.l }
	#[inline]
	fn close(&self) -> f64 { self.c }
	#[inline]
	fn buy_volume(&self) -> f64 { self.bv }
	#[inline]
	fn total_volume(&self) -> f64 { self.v }
}

pub fn aggregate<C: Candle>(data: &[C]) -> SetAggregate {
	let mut aggregate = SetAggregate::default();

	if data.len() == 0 {
		return aggregate;
	}

	aggregate.o = data[0].open();
	aggregate.h = data[0].high();
	aggregate.l = data[0].low();
	aggregate.c = data[data.len() - 1].close();

	for elem in data {
		let high = elem.high();
		let low = elem.low();

		if high > aggregate.h {
			aggregate.h = high;
		}
		if low < aggregate.l {
			aggregate.l = low;
		}

		aggregate.bv = elem.buy_volume();
		aggregate.v = elem.total_volume();
	}

	aggregate
}

const LEN_OF_MINUTE: u64 = 60;
const LEN_OF_HOUR: u64 = 60 * LEN_OF_MINUTE;
const LEN_OF_DAY: u64 = 24 * LEN_OF_HOUR;
const LEN_OF_WEEK: u64 = 7 * LEN_OF_DAY;
const LEN_OF_MONTH: u64 = 30 * LEN_OF_DAY;
const LEN_OF_YEAR: u64 = 365 * LEN_OF_DAY;

pub fn duration_string(elapsed: u64) -> String {
	if elapsed < 10 {
		return "Now".to_string();
	}

	let (secs, mins, hours, days, weeks, months, years) = (
		elapsed % LEN_OF_MINUTE,
		(elapsed % LEN_OF_HOUR) / LEN_OF_MINUTE,
		(elapsed % LEN_OF_DAY) / LEN_OF_HOUR,
		(((elapsed % LEN_OF_YEAR) % LEN_OF_MONTH) % LEN_OF_WEEK) / LEN_OF_DAY,
		((elapsed % LEN_OF_YEAR) % LEN_OF_MONTH) / LEN_OF_WEEK,
		(elapsed % LEN_OF_YEAR) / LEN_OF_MONTH,
		elapsed / LEN_OF_YEAR,
	);

	let mut elapsed_str = String::new();

	if years > 0 {
		elapsed_str += &format!("{}y", years);
	}
	if months > 0 {
		elapsed_str += &format!("{}m", months);
	}
	if weeks > 0 {
		elapsed_str += &format!("{}w", weeks);
	}
	if days > 0 {
		elapsed_str += &format!("{}d", days);
	}
	if hours > 0 {
		elapsed_str += &format!("{}h", hours);
	}
	if mins > 0 {
		elapsed_str += &format!("{}m", mins);
	}
	if secs > 0 {
		elapsed_str += &format!("{}s", secs);
	}

	elapsed_str
}
