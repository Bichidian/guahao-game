use crate::{
    action::{Action, Resource, INIT_STATE},
    game::{GameInfo, RoundOutcome},
};
use eframe::egui;
use std::sync::mpsc;
// use eframe::EventLoopBuilderHook;
// use winit::platform::wayland::EventLoopBuilderExtWayland;

pub struct GUIApp {
    state_receiver: mpsc::Receiver<GameInfo>,
    action_sender: mpsc::Sender<Action>,
    is_active: bool,
    state: Resource,
    other_state: Resource,
    action: Option<Action>,
    other_action: Option<Action>,
    outcome: RoundOutcome,
    is_legal_action: [bool; Self::ACTION_LIST.len()],
}

impl eframe::App for GUIApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Ok(game_info) = self.state_receiver.try_recv() {
                self.state = game_info.state;
                self.other_state = game_info.other_state;
                self.other_action = Some(game_info.other_action);
                self.outcome = game_info.outcome;
                if matches!(self.outcome, RoundOutcome::Continue) {
                    self.is_active = true;
                    self.update_legal_actions();
                }
            }

            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.label(self.other_state.to_string());
                if let Some(other_action) = self.other_action.as_ref() {
                    ui.label(format!("{}", other_action));
                } else {
                    ui.label("");
                }
                if matches!(self.outcome, RoundOutcome::Win | RoundOutcome::Lose) {
                    ui.add_space(ui.max_rect().size().y / 2.0 - ui.min_rect().size().y - 20.0);
                    ui.label(match self.outcome {
                        RoundOutcome::Continue => unreachable!(),
                        RoundOutcome::Win => "您赢了",
                        RoundOutcome::Lose => "您输了",
                    });
                }
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.allocate_ui_with_layout(
                    [40.0 * 5. + 8.0 * 4., 43.0].into(),
                    egui::Layout::left_to_right(egui::Align::TOP),
                    |ui| {
                        let mut action_legality_iter =
                            Self::ACTION_LIST.into_iter().zip(self.is_legal_action.into_iter());
                        if let Some((action, legality)) = action_legality_iter.next() {
                            self.add_action_button(ui, action, legality, [40.0, 43.0]);
                        }
                        while let (Some((action1, legality1)), Some((action2, legality2))) =
                            (action_legality_iter.next(), action_legality_iter.next())
                        {
                            ui.vertical(|ui| {
                                self.add_action_button(ui, action1, legality1, [40.0, 20.0]);
                                self.add_action_button(ui, action2, legality2, [40.0, 20.0]);
                            });
                        }
                    },
                );
                ui.label(self.state.to_string());
                if let Some(action) = self.action.as_ref() {
                    ui.label(format!("{}", action));
                } else {
                    ui.label("请出招");
                }
            });
        });
    }
}

impl GUIApp {
    const ACTION_LIST: [Action; 9] = [
        Action::Guahao,
        Action::Attack(1),
        Action::Defend(1),
        Action::Attack(2),
        Action::Defend(2),
        Action::Attack(3),
        Action::Defend(3),
        Action::Fantan,
        Action::Quanfang,
    ];

    fn update_legal_actions(&mut self) {
        for (action, legality) in Self::ACTION_LIST.iter().zip(self.is_legal_action.as_mut()) {
            let cost = action.get_cost();
            *legality = true;
            for (s, c) in self.state.iter().zip(cost.iter()) {
                if *s < *c {
                    *legality = false;
                    break;
                }
            }
        }
    }

    fn add_action_button(&mut self, ui: &mut egui::Ui, action: Action, legality: bool, size: impl Into<egui::Vec2>) {
        ui.add_enabled_ui(self.is_active && legality, |ui| {
            if ui.add_sized(size, egui::Button::new(action.to_string())).clicked() {
                self.action_sender.send(action).unwrap_or_else(|_| {
                    eprintln!("游戏已关闭");
                });
                self.action = Some(action);
                self.is_active = false;
            }
        });
    }

    fn new(state_receiver: mpsc::Receiver<GameInfo>, action_sender: mpsc::Sender<Action>) -> Self {
        let mut gui_app = Self {
            state_receiver,
            action_sender,
            is_active: true,
            state: INIT_STATE,
            other_state: INIT_STATE,
            action: None,
            other_action: None,
            outcome: RoundOutcome::Continue,
            is_legal_action: [false; Self::ACTION_LIST.len()],
        };
        gui_app.update_legal_actions();
        gui_app
    }

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

    pub fn run_gui(state_receiver: mpsc::Receiver<GameInfo>, action_sender: mpsc::Sender<Action>) {
        // let event_loop_builder: Option<EventLoopBuilderHook> = Some(Box::new(|event_loop_builder| {
        //     event_loop_builder.with_any_thread(true);
        // }));
        let native_options = eframe::NativeOptions {
            // event_loop_builder,
            viewport: egui::ViewportBuilder::default().with_inner_size((800.0, 600.0)),
            ..eframe::NativeOptions::default()
        };
        eframe::run_native(
            "挂号游戏",
            native_options,
            Box::new(|cc| {
                cc.egui_ctx.set_zoom_factor(2.0);
                Self::set_font(cc, include_bytes!("../fonts/NotoSansSC-Regular.otf"));
                Ok(Box::new(Self::new(state_receiver, action_sender)))
            }),
        )
        .unwrap();
    }
}
