


use eframe::egui::{self, TextBuffer as _};



fn main() {
    eframe::run_native(
        "Design Project",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder {
                inner_size: Some([1200.0, 800.0].into()),
                ..Default::default()
            },
            ..Default::default()
        },
        Box::new(move |cc| Ok(Box::new(Program::new(cc)))),
    ).unwrap();
}



struct Program {
    running_seats: Vec<Seat>,

    seat_name_field_buf: String,
}

impl Program {
    fn new(cc: &eframe::CreationContext) -> Self {
        let mut visuals = egui::Visuals::dark();
        visuals.button_frame = false;
        visuals.interact_cursor = Some(egui::CursorIcon::PointingHand);
        visuals.override_text_color = Some(egui::Color32::from_rgb(0xaa, 0xaa, 0xab));
        visuals.extreme_bg_color = egui::Color32::from_rgb(0x1e, 0x1e, 0x22);
        visuals.window_fill = egui::Color32::from_rgb(0x1e, 0x1e, 0x22);
        visuals.panel_fill = egui::Color32::from_rgb(0x2b, 0x2b, 0x33);
        cc.egui_ctx.set_visuals(visuals);

        cc.egui_ctx.style_mut(|s| {
            use egui::{FontFamily, FontId, TextStyle::*};
            s.text_styles = [
                (Heading, FontId::new(29.0, FontFamily::Proportional)),
                (Body, FontId::new(19.0, FontFamily::Proportional)),
                (Monospace, FontId::new(19.0, FontFamily::Monospace)),
                (Button, FontId::new(19.0, FontFamily::Proportional)),
                (Small, FontId::new(13.0, FontFamily::Proportional)),
            ].into();
        });

        Self {
            running_seats: vec![],
            seat_name_field_buf: String::new(),
        }
    }
}

impl eframe::App for Program {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.text_edit_singleline(&mut self.seat_name_field_buf);
            if ui.button("Run").clicked() {
                self.running_seats.push(Seat::new(self.seat_name_field_buf.take()));
            }
        });

        for seat in self.running_seats.iter_mut() {
            ctx.show_viewport_immediate(
                seat.id,
                egui::ViewportBuilder::default()
                    .with_title(&seat.name),
                |ctx, _class| {
                    seat.update(ctx);
                },
            );
        }
    }
}



struct Seat {
    id: egui::ViewportId,
    name: String,
}

impl Seat {
    fn new(name: String) -> Self {
        let id = egui::ViewportId::from_hash_of(&name);

        Self {
            id,
            name,
        }
    }

    fn update(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| ui.heading(&self.name));
        });
    }
}
