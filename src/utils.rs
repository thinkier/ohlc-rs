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