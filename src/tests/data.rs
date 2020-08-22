use std::hash::{Hash, Hasher};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct OHLC {
    pub o: f64,
    pub h: f64,
    pub l: f64,
    pub c: f64,
}

impl Hash for OHLC {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.o as u64);
        state.write_u64(self.h as u64);
        state.write_u64(self.l as u64);
        state.write_u64(self.c as u64);
    }
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

    pub fn range(&self) -> f64 {
        (self.h - self.l).abs()
    }
}