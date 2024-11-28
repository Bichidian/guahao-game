use crate::{
    action::{Action, Resource, INIT_STATE},
    game::RoundOutcome,
    player::GameFeedback,
};
use eframe::egui;
use std::sync::mpsc;
// use eframe::EventLoopBuilderHook;
// use winit::platform::wayland::EventLoopBuilderExtWayland;

pub struct GUIApp {
    state_receiver: mpsc::Receiver<GameFeedback>,
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
            ui.heading("挂号游戏");
            if let Ok(game_feedback) = self.state_receiver.try_recv() {
                self.state = game_feedback.state;
                self.other_state = game_feedback.other_state;
                self.other_action = Some(game_feedback.other_action);
                self.outcome = game_feedback.outcome;
                if matches!(self.outcome, RoundOutcome::Continue) {
                    self.is_active = true;
                    self.update_legal_actions();
                }
            }
            ui.label(format!(
                "我方剩余：挂号{}，全防{}，反弹{}。对方剩余：挂号{}，全防{}，反弹{}。",
                self.state[0],
                self.state[1],
                self.state[2],
                self.other_state[0],
                self.other_state[1],
                self.other_state[2]
            ));
            if let (Some(action), Some(other_action)) = (self.action.as_ref(), self.other_action.as_ref()) {
                ui.label(format!("我方出招：{}，对方出招：{}。", action, other_action));
            } else {
                ui.label("请出招");
            }
            match self.outcome {
                RoundOutcome::Continue => ui.label("加油！"),
                RoundOutcome::Win => ui.label("您赢了！"),
                RoundOutcome::Lose => ui.label("您输了！"),
            };

            for (action, legality) in Self::ACTION_LIST.iter().zip(self.is_legal_action.iter()) {
                if ui
                    .add_enabled(self.is_active && *legality, egui::Button::new(action.to_string()))
                    .clicked()
                {
                    self.action_sender.send(action.clone()).unwrap_or_else(|_| {
                        eprintln!("游戏已关闭");
                    });
                    self.action = Some(action.clone());
                    self.is_active = false;
                }
            }
        });
    }
}

impl GUIApp {
    const ACTION_LIST: [Action; 9] = [
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

    fn new(state_receiver: mpsc::Receiver<GameFeedback>, action_sender: mpsc::Sender<Action>) -> Self {
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

    pub fn run_gui(state_receiver: mpsc::Receiver<GameFeedback>, action_sender: mpsc::Sender<Action>) {
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
                Ok(Box::new(Self::new(state_receiver, action_sender)))
            }),
        )
        .unwrap();
    }
}
