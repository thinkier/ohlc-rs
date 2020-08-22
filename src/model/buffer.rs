pub use OHLCRenderOptions;

use super::*;

pub struct ChartBuffer {
    /// Total width for the graph
    width: usize,
    /// Total height for the graph
    height: usize,
    /// Margin for the actual graph
    pub margin: Margin,
    /// Maximum price the graph is able to display
    pub max_price: f64,
    /// Minimum price the graph is able to display
    pub min_price: f64,
    /// The amount of time the graph covers, in seconds
    pub timeframe: i64,
    /// Default background colour, alpha channel is ignored
    pub background: u32,
    /// Byte buffer of the actual image
    pub buffer: Vec<u8>,
}

impl ChartBuffer {
    pub(crate) fn new(width: usize, height: usize, margin: Margin, max_price: f64, min_price: f64, timeframe: i64, background: u32) -> ChartBuffer {
        if max_price < min_price {
            panic!("max < min... wut?");
        }

        if timeframe <= 0 {
            panic!("timeframe must be > 0");
        }

        if margin.top + margin.bottom > height || margin.left + margin.right > width {
            panic!("margins cannot be bigger than the image itself")
        }

        let mut buffer = Vec::with_capacity(width * height * 3);

        Self::colour_buffer(&mut buffer, width * height, background);

        ChartBuffer { width, height, margin, max_price, min_price, timeframe, background: background | 0xFF, buffer }
    }

    /// Returns: (x, y)
    pub fn data_to_coords(&self, price: f64, time: i64) -> Point {
        let x = {
            let prog = time as f64 / self.timeframe as f64;

            if prog <= 0. {
                self.margin.left
            } else if prog >= 1. {
                self.width - self.margin.right
            } else {
                self.margin.left + (prog * (self.width - (self.margin.right + self.margin.left)) as f64) as usize
            }
        };

        let y = {
            let prog = (price - self.min_price) / (self.max_price - self.min_price);

            if prog >= 1. {
                self.margin.top
            } else {
                let bottom = self.height - self.margin.bottom;

                if prog <= 0. {
                    bottom
                } else {
                    (bottom as f64 - (prog * (bottom - self.margin.top) as f64)) as usize
                }
            }
        };

        (x, y)
    }

    pub fn create_extension_strip<F>(&mut self, height: usize, f: F) where F: Fn(&mut ExtensionStrip) {
        // Have enough room for labels on the top, bottom and right
        let margin = Margin { top: 40, bottom: 35, left: self.margin.left, right: self.margin.right };

        self.height += height;
        self.margin.bottom += height;

        let mut es = ExtensionStrip::new(self.width, height, self.background, self.timeframe, margin);

        (f)(&mut es);

        self.buffer.extend(es.buffer);
    }

    pub fn put(&mut self, price: f64, time: i64, rgba: u32) {
        let (x, y) = self.data_to_coords(price, time);
        self.colour(x, y, rgba);
    }
}

impl Painter for ChartBuffer {
    fn buffer<'a>(&'a mut self) -> &mut Vec<u8> {
        &mut self.buffer
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn background(&self) -> u32 {
        self.background
    }
}

pub struct ExtensionStrip {
    width: usize,
    height: usize,
    pub background: u32,
    pub timeframe: i64,
    pub margin: Margin,
    pub buffer: Vec<u8>,
}

impl ExtensionStrip {
    pub fn new(width: usize, height: usize, background: u32, timeframe: i64, margin: Margin) -> ExtensionStrip {
        let mut buffer = Vec::with_capacity(width * height * 3);

        Self::colour_buffer(&mut buffer, width * height, background);

        ExtensionStrip {
            width,
            height,
            background,
            timeframe,
            margin,
            buffer,
        }
    }

    pub fn data_to_coords(&self, up_progress: f64, time: i64) -> Point {
        let x = {
            let prog = time as f64 / self.timeframe as f64;

            if prog <= 0. {
                self.margin.left
            } else if prog >= 1. {
                self.width - self.margin.right
            } else {
                self.margin.left + (prog * (self.width - (self.margin.right + self.margin.left)) as f64) as usize
            }
        };

        let y = {
            if up_progress >= 1. {
                self.margin.top
            } else {
                let bottom = self.height - self.margin.bottom;

                if up_progress <= 0. {
                    bottom
                } else {
                    (bottom as f64 - (up_progress * (bottom - self.margin.top) as f64)) as usize
                }
            }
        };

        (x, y)
    }
}

impl Painter for ExtensionStrip {
    fn buffer<'a>(&'a mut self) -> &mut Vec<u8> {
        &mut self.buffer
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn background(&self) -> u32 {
        self.background
    }
}
