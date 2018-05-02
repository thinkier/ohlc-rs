use super::*;
use super::*;

pub fn calculate_ohlc_of_set(data: &[OHLC]) -> OHLC {
	let mut ohlc = OHLC::new();

	if data.len() == 0 {
		return ohlc;
	}

	ohlc.o = data[0].o;
	ohlc.h = data[0].h;
	ohlc.l = data[0].l;
	ohlc.c = data[data.len() - 1].c;

	for elem in data {
		if elem.h > ohlc.h {
			ohlc.h = elem.h;
		}
		if elem.l < ohlc.l {
			ohlc.l = elem.l;
		}
	}

	ohlc
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

#[cfg(test)]
mod tests {
	#[test]
	fn duration_test() {
		assert_eq!(duration_string(1), "Now");
		assert_eq!(duration_string(10), "10s");
		assert_eq!(duration_string(60), "1m");
		assert_eq!(duration_string(61), "1m1s");
		assert_eq!(duration_string(3600), "1h");
		assert_eq!(duration_string(3661), "1h1m1s");
		assert_eq!(duration_string(86400), "1d");
		assert_eq!(duration_string(86400 + 3661), "1d1h1m1s");
		assert_eq!(duration_string(604800), "1w");
		assert_eq!(duration_string(604800 + 86400 + 3661), "1w1d1h1m1s");
		assert_eq!(duration_string(2592000), "1m");
		assert_eq!(duration_string(2592000 + 604800 + 86400 + 3661), "1m1w1d1h1m1s");
		assert_eq!(duration_string(365 * 86400), "1y");
		assert_eq!(duration_string(365 * 86400 + 2592000 + 604800 + 86400 + 3661), "1y1m1w1d1h1m1s");
	}
}