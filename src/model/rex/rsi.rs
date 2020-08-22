use model::*;

#[derive(Clone, Debug)]
pub struct RSI {
    label_colour: u32,
    colour: u32,
    overbought_colour: u32,
    oversold_colour: u32,
}

impl RSI {
    pub fn new(label_colour: u32, colour: u32, overbought_colour: u32, oversold_colour: u32) -> RSI {
        RSI { label_colour, colour, overbought_colour, oversold_colour }
    }
}

impl RendererExtension for RSI {
    fn apply(&self, buffer: &mut ChartBuffer, data: &[OHLC]) {
        let periods = 10;

        let mut rsi = vec![];

        for i in periods..data.len() {
            let rs = {
                let mut gains = vec![];
                let mut losses = vec![];

                for j in i - periods..i {
                    let delta = data[j].c - data[j].o;
                    if delta >= 0. {
                        gains.push(delta);
                        losses.push(0.);
                    } else {
                        losses.push(delta.abs());
                        gains.push(0.);
                    }
                }

                avg(&gains[..]) / avg(&losses[..])
            };

            rsi.push(100. - 100. / (1. + rs));
        }

        buffer.create_extension_strip(175, move |buffer| {
            buffer.text((8, 8), "RSI(10)", self.label_colour);

            // Lines and labels
            {
                {
                    let p1 = buffer.data_to_coords(0.7, 0);
                    let p2 = buffer.data_to_coords(0.7, buffer.timeframe);

                    buffer.line(p1, p2, self.overbought_colour);
                    buffer.text((p2.0 + 4, p2.1 - 8), "Overbought", self.overbought_colour);
                }
                {
                    let p1 = buffer.data_to_coords(0.3, 0);
                    let p2 = buffer.data_to_coords(0.3, buffer.timeframe);

                    buffer.line(p1, p2, self.oversold_colour);
                    buffer.text((p2.0 + 4, p2.1 - 8), "Oversold", self.oversold_colour);
                }

                for prog in [0, 50, 100].iter() {
                    let p1 = buffer.data_to_coords(*prog as f64 / 100., 0);
                    let p2 = buffer.data_to_coords(*prog as f64 / 100., buffer.timeframe);

                    buffer.line(p1, p2, self.label_colour);
                    buffer.text((p2.0 + 4, p2.1 - 8), &format!("{}", prog), self.label_colour);
                }
            }

            // Actual RSI Curve
            {
                let offset = ((periods as f64 + 0.5) * (buffer.timeframe as f64) / (data.len() as f64)) as i64;

                for i in 0..rsi.len() - 1 {
                    let p1 = buffer.data_to_coords(rsi[i] / 100., buffer.timeframe * i as i64 / data.len() as i64 + offset);
                    let p2 = buffer.data_to_coords(rsi[i + 1] / 100., buffer.timeframe * (i + 1) as i64 / data.len() as i64 + offset);

                    buffer.line(p1, p2, self.colour);
                }
            }
        });
    }

    fn lore_colour(&self) -> Option<u32> {
        None
    }

    fn name(&self) -> String {
        "RSI(10)".to_string()
    }
}

fn avg(prices: &[f64]) -> f64 {
    let mut sum = 0.;

    for price in prices {
        sum += *price;
    }

    sum / prices.len() as f64
}
