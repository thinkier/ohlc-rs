use model::*;

#[derive(Clone, Debug)]
pub struct OHLCCandles {
    up_colour: u32,
    down_colour: u32,
}

impl OHLCCandles {
    pub fn new(up_colour: u32, down_colour: u32) -> OHLCCandles {
        OHLCCandles { up_colour, down_colour }
    }
}

impl RendererExtension for OHLCCandles {
    fn apply(&self, buffer: &mut ChartBuffer, data: &[OHLC]) {
        let period = buffer.timeframe / data.len() as i64;
        let period_addition = 4. * period as f64 / 5.;

        for i in 0..data.len() {
            let ohlc = data[i];

            let colour = if ohlc.o > ohlc.c { self.down_colour } else { self.up_colour };

            // Main big block
            {
                let p1 = buffer.data_to_coords(ohlc.o, period * i as i64);
                let p2 = buffer.data_to_coords(ohlc.c, ((period * (i as i64)) as f64 + period_addition) as i64);

                buffer.rect_point(p1, p2, colour);
            }

            // Sticks
            {
                let time = period * i as i64 + (period_addition / 2.) as i64;
                let p1 = buffer.data_to_coords(ohlc.h, time - (period_addition / 12.).ceil() as i64);
                let p2 = buffer.data_to_coords(ohlc.l, time + (period_addition / 12.).floor() as i64);

                buffer.rect_point(p1, p2, colour);
            }
        }
    }

    fn lore_colour(&self) -> Option<u32> {
        None
    }

    fn name(&self) -> String {
        "OHLC_Candles()".to_string()
    }
}
