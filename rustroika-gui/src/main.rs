use rustroika_core::{CalculationConfig, CalculationResult, calculate_transport_costs};
use eframe::egui;
use std::sync::Arc;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions { 
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([600.0, 400.0])
            .with_title("Rustroika")
            .with_icon(Arc::new(create_icon())),
        ..Default::default()
    };

    eframe::run_native(
        "rustroika", 
        options, 
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx);
            Ok(Box::new(RustroikaApp::default()))
        }),
    )
}

fn setup_custom_fonts(egui_ctx: &egui::Context) -> _ {
    todo!()
}

fn create_icon() -> egui::IconData {
    let icon_size = 32;
    let mut rgba = vec![0u8, icon_size * icon_size + 4];

    for (i, pixel) in rgba.chunks_mut(4).enumerate() {
        let x = i % icon_size;
        let y = i / icon_size;

        let r = ((x as f32 / icon_size as f32) * 255.0) as u8;
        let g = ((y as f32 / icon_size as f32) * 255.0) as u8;
        let b = 100u8;
        let a = 255u8;

        pixel[0] = r;
        pixel[1] = g;
        pixel[2] = b;
        pixel[3] = a;
    };

    eframe::IconData {
        rgba,
        width: icon_size as u32,
        height: icon_size as u32,
    }
}

#[derive(Default)]
struct RustroikaApp {}
