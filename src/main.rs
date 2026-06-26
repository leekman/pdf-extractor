mod app;
mod ui;

use eframe::egui;

use app::App;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "PDF Extractor",
        options,
        Box::new(|cc| {
            configure_style(&cc.egui_ctx);
            Ok(Box::<App>::default())
        }),
    )
}

fn configure_style(ctx: &egui::Context) {
    let mut style = (*ctx.global_style()).clone();

    let rounding = egui::CornerRadius::same(6);
    style.visuals.widgets.noninteractive.corner_radius = rounding;
    style.visuals.widgets.inactive.corner_radius = rounding;
    style.visuals.widgets.hovered.corner_radius = rounding;
    style.visuals.widgets.active.corner_radius = rounding;

    style.spacing.item_spacing = egui::vec2(10.0, 8.0);
    style.spacing.button_padding = egui::vec2(14.0, 6.0);

    ctx.set_global_style(style);
}
