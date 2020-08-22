use model::*;
use model::buffer::ChartBuffer;
use model::rex::dema::*;
use model::rex::ema::*;

#[derive(Clone, Debug)]
pub struct MACD {
    divergence_colour: u32,
    signal_colour: u32,
    histogram_colour: u32,
    label_colour: u32,
    smoothing_factor: f64,
}

impl MACD {
    pub fn new(divergence_colour: u32, signal_colour: u32, histogram_colour: u32, label_colour: u32, smoothing_factor: f64) -> MACD {
        MACD {
            divergence_colour,
            signal_colour,
            histogram_colour,
            label_colour,
            smoothing_factor,
        }
    }
}

impl RendererExtension for MACD {
    fn apply(&self, buffer: &mut ChartBuffer, data: &[OHLC]) {
        let (divergence, signal, histogram) = {
            let median_list = median_list(data);

            let short = ema(&EMA::new(12, self.smoothing_factor, 0), &median_list);
            let long = ema(&EMA::new(26, self.smoothing_factor, 0), &median_list);

            let mut divergence = short.clone();
            subtract(&mut divergence, &long);

            let signal = ema(&EMA::new(9, self.smoothing_factor, 0), &divergence);

            let mut histogram = divergence.clone();
            subtract(&mut histogram, &signal);

            (divergence, signal, histogram)
        };

        let (mut lowest, mut highest) = (divergence[0], divergence[0]);

        for set in &[&divergence, &signal, &histogram] {
            for number in *set {
                if *number < lowest {
                    lowest = *number;
                }
                if *number > highest {
                    highest = *number;
                }
            }
        }

        if lowest > 0. {
            lowest = 0.;
        }

        let range = highest - lowest;

        buffer.create_extension_strip(135, move |buffer| {
            buffer.text((8, 8), &self.name(), self.label_colour);
            buffer.text_with_background((8, 8 + 17), "MACD Divergence", self.divergence_colour, 0x7F7F7F7F);
            buffer.text_with_background((8, 8 + 17 * 2), "MACD Signal", self.signal_colour, 0x7F7F7F7F);

            let period = buffer.timeframe / data.len() as i64;
            let period_addition = 4. * period as f64 / 5.;

            // Histogram
            {
                for i in 35..histogram.len() {
                    let time = period * i as i64 + (period_addition / 2.) as i64;
                    let p1 = buffer.data_to_coords((histogram[i] - lowest) / range, time - (period_addition / 12.).ceil() as i64);
                    let p2 = buffer.data_to_coords(-lowest / range, time + (period_addition / 12.).floor() as i64);

                    buffer.rect_point(p1, p2, self.histogram_colour);
                }
            }

            // Zero line
            {
                let prog = -lowest / range;
                let p1 = buffer.data_to_coords(prog, 0);
                let p2 = buffer.data_to_coords(prog, buffer.timeframe);

                buffer.line(p1, p2, self.label_colour);
                buffer.text((p2.0 + 4, p2.1 - 8), "Zero", self.label_colour);
            }

            // Signal & divergence
            {
                for (data, colour, begin_pos) in &[(&signal, self.signal_colour, 35), (&divergence, self.divergence_colour, 26)] {
                    let len = data.len() as i64;

                    for i in *begin_pos + 1..len as usize {
                        let time1 = period * (i - 1) as i64 + (period_addition / 2.) as i64;
                        let time2 = period * i as i64 + (period_addition / 2.) as i64;
                        let p1 = buffer.data_to_coords((data[i - 1] - lowest) / range, time1 - (period_addition / 12.).ceil() as i64);
                        let p2 = buffer.data_to_coords((data[i] - lowest) / range, time2 - (period_addition / 12.).floor() as i64);

                        buffer.line(p1, p2, *colour);
                    }
                }
            }
        });
    }

    fn lore_colour(&self) -> Option<u32> {
        None
    }

    fn name(&self) -> String {
        format!("MACD(12, 26, 9, sf={})", self.smoothing_factor)
    }
}
