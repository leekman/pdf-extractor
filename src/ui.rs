use eframe::egui;
use std::fs;

use crate::app::{App, Status};

const SUCCESS: egui::Color32 = egui::Color32::from_rgb(143, 191, 122);
const ERROR: egui::Color32 = egui::Color32::from_rgb(220, 100, 100);
const DIM: egui::Color32 = egui::Color32::from_rgb(120, 128, 140);

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::bottom("status").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                match &self.status {
                    Status::Idle => {
                        ui.label(egui::RichText::new("Prêt").size(12.0).color(DIM));
                    }
                    Status::Success(msg) => {
                        ui.label(
                            egui::RichText::new(format!("✓  {}", msg))
                                .size(12.0)
                                .color(SUCCESS),
                        );
                    }
                    Status::Error(msg) => {
                        ui.label(
                            egui::RichText::new(format!("✗  {}", msg))
                                .size(12.0)
                                .color(ERROR),
                        );
                    }
                }
            });
        });

        ui.add_space(4.0);

        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("PDF Extractor").size(18.0).strong());

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .button(egui::RichText::new("💾  Enregistrer").size(13.0))
                    .clicked()
                {
                    let default_name = self.default_output_name();
                    if let Some(path) = rfd::FileDialog::new()
                        .set_file_name(default_name)
                        .add_filter("Markdown", &["md"])
                        .save_file()
                    {
                        match fs::write(&path, &self.markdown) {
                            Ok(()) => {
                                self.status =
                                    Status::Success(format!("Enregistré : {}", path.display()));
                            }
                            Err(e) => {
                                self.status = Status::Error(format!("{}", e));
                            }
                        }
                    }
                }

                if ui
                    .button(egui::RichText::new("📂  Ouvrir un PDF").size(13.0))
                    .clicked()
                {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("PDF", &["pdf"])
                        .pick_file()
                    {
                        self.convert_pdf(&path);
                    }
                }
            });
        });

        ui.add_space(4.0);
        ui.separator();
        ui.add_space(4.0);

        if self.markdown.is_empty() {
            ui.vertical_centered(|ui| {
                ui.add_space(ui.available_height() / 3.0);
                ui.label(egui::RichText::new("Glissez un PDF ici").size(18.0).color(DIM));
                ui.add_space(6.0);
                ui.label(
                    egui::RichText::new("ou cliquez sur « Ouvrir un PDF »")
                        .size(13.0)
                        .color(DIM),
                );
            });
        } else {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut self.markdown)
                        .font(egui::TextStyle::Monospace)
                        .desired_width(f32::INFINITY),
                );
            });
        }

        let dropped_path = ui
            .ctx()
            .input(|i| i.raw.dropped_files.first().and_then(|f| f.path.clone()));

        if let Some(path) = dropped_path {
            self.convert_pdf(&path);
        }
    }
}
