use eframe::egui;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Vizsla",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp {
    show_confirmation_dialog: bool,
    allowed_to_close: bool,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut s: String = String::from("Hello World");
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Welcome");
            if ui.button(&s).clicked(){
                s = String::from("worldwars");
            }
        });

        
        // if ctx.input(|i| i.viewport().close_requested()) {
        //     if self.allowed_to_close {
        //         // do nothing - we will close
        //     } else {
        //         ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
        //         self.show_confirmation_dialog = true;
        //     }
        // }

        // if self.show_confirmation_dialog {
        //     egui::Window::new("Do you want to quit?")
        //         .collapsible(false)
        //         .resizable(false)
        //         .show(ctx, |ui| {
        //             ui.horizontal(|ui| {
        //                 if ui.button("No").clicked() {
        //                     self.show_confirmation_dialog = false;
        //                     self.allowed_to_close = false;
        //                 }

        //                 if ui.button("Yes").clicked() {
        //                     self.show_confirmation_dialog = false;
        //                     self.allowed_to_close = true;
        //                     ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
        //                 }
        //             });
        //         });
        // }
    }
}