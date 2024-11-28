use crate::action::{Action, Resource};
use eframe::egui;
use std::sync::mpsc;
// use eframe::EventLoopBuilderHook;
// use winit::platform::wayland::EventLoopBuilderExtWayland;

pub struct GUIApp {
    state_receiver: mpsc::Receiver<[Resource; 2]>,
    action_sender: mpsc::Sender<Action>,
}

impl eframe::App for GUIApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let button_action_list = [
            Action::Guahao,
            Action::Attack(1),
            Action::Attack(2),
            Action::Attack(3),
            Action::Defend(1),
            Action::Defend(2),
            Action::Defend(3),
            Action::Quanfang,
            Action::Fantan,
        ];
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("挂号游戏");
            for action in button_action_list {
                if ui.button(action.to_string()).clicked() {
                    if self.state_receiver.try_recv().is_ok() {
                        self.action_sender.send(action).unwrap();
                    }
                }
            }
        });
    }
}

impl GUIApp {
    fn set_font(cc: &eframe::CreationContext<'_>, font: &'static [u8]) {
        let mut fonts = egui::FontDefinitions::default();
        fonts
            .font_data
            .insert("chinese_font".to_owned(), egui::FontData::from_static(font));
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "chinese_font".to_owned());
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .push("chinese_font".to_owned());
        cc.egui_ctx.set_fonts(fonts);
    }

    pub fn run_gui(state_receiver: mpsc::Receiver<[[i8; 3]; 2]>, action_sender: mpsc::Sender<Action>) {
        // let event_loop_builder: Option<EventLoopBuilderHook> = Some(Box::new(|event_loop_builder| {
        //     event_loop_builder.with_any_thread(true);
        // }));
        let native_options = eframe::NativeOptions {
            // event_loop_builder,
            viewport: egui::ViewportBuilder::default().with_inner_size((400.0, 400.0)),
            ..eframe::NativeOptions::default()
        };
        eframe::run_native(
            "挂号游戏",
            native_options,
            Box::new(|cc| {
                Self::set_font(cc, include_bytes!("../fonts/NotoSansSC-Regular.otf"));
                Ok(Box::new(GUIApp {
                    state_receiver,
                    action_sender,
                }))
            }),
        )
        .unwrap();
    }
}
