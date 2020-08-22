extern crate env_logger;
extern crate serde_json;

use model::rex::*;
use model::rex::test_fill::TestFill;
use model::rex::test_line::TestLine;
use model::rex::test_text::TestText;
use tests::data::OHLC;

use super::*;

mod data;

fn draw_with_extension<T: RendererExtension + 'static>(ext: Option<T>, suffix: &str) {
    let _ = env_logger::try_init();

    let data: Vec<OHLC> = self::serde_json::from_str(include_str!("../sample_data.json")).unwrap();

    {
        let mut options = OHLCRenderOptions::new();
        options.title(&format!("BTCUSD | ohlc-rs{}", suffix), 0x007F7FFF)
            .line(0xCCCCCCFF, 200., 24)
            .background_colour(0x36393EFF);

        if let Some(ext) = ext {
            options.add_extension(ext);
        }

        options.render_and_save(
            data.clone(),
            &Path::new(&format!("test-draw-sample-data{}.png", suffix)),
        ).unwrap();
    }
}

#[test]
fn render_draw_sample_data() {
    draw_with_extension::<NoExtension>(None, "");
}

#[test]
fn render_draw_sample_data_plus_bb() {
    draw_with_extension(Some(BollingerBands::new(20, 2, 0xFF0000FF)), "+bb");
}

#[test]
fn render_draw_sample_data_plus_dema() {
    draw_with_extension(Some(DEMA::new(EMA::new(20, 0.1, 0xFF0000FF))), "+dema");
}

#[test]
fn render_draw_sample_data_plus_ema() {
    draw_with_extension(Some(EMA::new(20, 0.1, 0xFF0000FF)), "+ema");
}

#[test]
fn render_draw_sample_data_plus_macd() {
    draw_with_extension(Some(MACD::new(0xFF007FFF, 0xFFFFFFFF, 0x00FFFFFF, 0xFF0000FF, 0.1)), "+macd");
}

#[test]
fn render_draw_sample_data_plus_rsi() {
    draw_with_extension(Some(RSI::new(0xCCCCCCFF, 0xFFFF007F, 0x27A819FF, 0xD33040FF)), "+rsi");
}

#[test]
fn render_draw_sample_data_plus_volume() {
    let data: Vec<OHLC> = self::serde_json::from_str(include_str!("../sample_data.json")).unwrap();

    let mut volumes = vec![];

    for ohlc in data {
        volumes.push(ohlc.h - 5000.);
    }

    draw_with_extension(Some(Volume::new(0xCCCCCCFF, volumes, 0x27A819FF, 0xD33040FF)), "+volume");
}

#[test]
fn render_draw_sample_data_with_test_text() {
    draw_with_extension(Some(TestText {}), "_test_text");
}

#[test]
fn render_draw_sample_data_with_test_fill() {
    draw_with_extension(Some(TestFill { colour: 0xFFFF00FF }), "_test_fill");
}

#[test]
fn render_draw_sample_data_with_test_fill_alpha() {
    draw_with_extension(Some(TestFill { colour: 0xFFFF007F }), "_test_fill_alpha");
}

#[test]
fn render_draw_sample_data_with_test_line() {
    draw_with_extension(Some(TestLine {}), "_with_test_line");
}
