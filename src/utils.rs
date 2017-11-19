use super::*;

pub fn calculate_ohlc_of_set(data: &Vec<OHLC>) -> OHLC {
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

pub fn duration_string(elapsed: u64) -> String {
	if elapsed < 10 {
		return "Now".to_string();
	}

	let (secs, mins, hours, days, weeks, months, years) = (
		elapsed % 60,
		(elapsed % 3600) / 60,
		(elapsed % 86400) / 3600,
		(elapsed % 604800) / 86400,
		(elapsed % 2592000) / 604800,
		(elapsed % 31557600) / 2592000,
		elapsed / 31557600,
	);

	let mut elapsed_str = String::new();

	if years > 0 {
		elapsed_str += &format!("{}y", years);
	}
	if months > 0 {
		elapsed_str += &format!("{}m", months);
	}
	if weeks > 0 {
		elapsed_str += &format!("{}d", weeks);
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
