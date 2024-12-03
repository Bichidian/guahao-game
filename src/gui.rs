use crate::{
    action::{Action, Resource, INIT_STATE},
    game::{Game, GameInfo, RoundOutcome},
    player::{BotPlayer, GUIPlayer},
};
use eframe::egui;
use std::{collections::BTreeMap, sync::mpsc};

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

        egui::TopBottomPanel::bottom("button_panel")
            .frame(egui::Frame {
                inner_margin: egui::Margin::symmetric(8.0, 8.0),
                fill: egui::Color32::from_gray(27),
                ..Default::default()
            })
            .show(ctx, |ui| {
                self.add_action_buttons(ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                Self::show_state(ui, self.other_state);
                if let Some(action) = self.other_action {
                    Self::show_action(ui, action);
                }

                if matches!(self.outcome, RoundOutcome::Win | RoundOutcome::Lose) {
                    let font_size = 36.0;
                    ui.add_space(ui.max_rect().size().y / 2.0 - ui.min_rect().size().y - font_size / 2.0 - 15.0);
                    let (text, color) = match self.outcome {
                        RoundOutcome::Continue => unreachable!(),
                        RoundOutcome::Win => ("胜", egui::Color32::GOLD),
                        RoundOutcome::Lose => ("负", egui::Color32::BROWN),
                    };
                    ui.label(Self::create_text(text, "wenkai", font_size).color(color));
                    if ui.button(Self::create_text("再来一局", "noto", 12.5)).clicked() {
                        *self = Self::new();
                    }
                }
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                Self::show_state(ui, self.state);
                if let Some(action) = self.action {
                    Self::show_action(ui, action);
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

    fn get_action_color(action: Action) -> egui::Color32 {
        match action {
            Action::Guahao => egui::Color32::from_rgb(135, 206, 235), // Sky blue
            Action::Attack(_) => egui::Color32::LIGHT_RED,
            Action::Defend(_) => egui::Color32::LIGHT_GREEN,
            Action::Fantan | Action::Quanfang => egui::Color32::ORANGE,
        }
    }

    fn add_action_buttons(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            let button_spacing = 5.0;
            ui.allocate_ui_with_layout(
                [40.0 * 5. + button_spacing * 4., 40.0 + button_spacing].into(),
                egui::Layout::left_to_right(egui::Align::TOP),
                |ui| {
                    ui.style_mut().spacing.item_spacing = [button_spacing, button_spacing].into();
                    let mut action_legality_iter = Self::ACTION_LIST.into_iter().zip(self.is_legal_action.into_iter());
                    if let Some((action, legality)) = action_legality_iter.next() {
                        self.add_single_action_button(ui, action, legality, [40.0, 40.0 + button_spacing]);
                    }
                    while let (Some((action1, legality1)), Some((action2, legality2))) =
                        (action_legality_iter.next(), action_legality_iter.next())
                    {
                        ui.vertical(|ui| {
                            self.add_single_action_button(ui, action1, legality1, [40.0, 20.0]);
                            self.add_single_action_button(ui, action2, legality2, [40.0, 20.0]);
                        });
                    }
                },
            );
        });
    }

    fn add_single_action_button(
        &mut self,
        ui: &mut egui::Ui,
        action: Action,
        legality: bool,
        size: impl Into<egui::Vec2>,
    ) {
        let color = Self::get_action_color(action);
        let text = Self::create_text(&action.to_string(), "noto", 12.5);

        ui.add_enabled_ui(self.is_active && legality, |ui| {
            ui.style_mut().visuals.widgets.inactive.fg_stroke.color = color;
            ui.style_mut().visuals.widgets.inactive.bg_stroke = (1.0, color).into();
            if ui.add_sized(size, egui::Button::new(text)).clicked() {
                self.action_sender.send(action).unwrap_or_else(|_| {
                    eprintln!("游戏已关闭");
                });
                self.action = Some(action);
                self.is_active = false;
            }
        });
    }

    fn show_action(ui: &mut egui::Ui, action: Action) {
        ui.add_space(5.0);
        let color = Self::get_action_color(action);
        ui.label(Self::create_text(&action.to_string(), "smiley", 25.0).color(color));
    }

    fn show_state(ui: &mut egui::Ui, state: Resource) {
        let text = format!("挂号 {}     全防 {}     反弹 {}", state[0], state[1], state[2]);
        ui.label(Self::create_text(&text, "wenkai", 15.0).color(egui::Color32::GRAY));
    }

    fn create_text(text: &str, family: &str, size: f32) -> egui::RichText {
        egui::RichText::new(text)
            .family(egui::FontFamily::Name(family.into()))
            .size(size)
    }

    fn new() -> Self {
        let (gui_player, state_receiver, action_sender) = GUIPlayer::new();
        std::thread::spawn(move || Game::new().run_game(gui_player, BotPlayer));
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

    fn add_fonts(cc: &eframe::CreationContext<'_>) {
        const NOTO: &[u8] = include_bytes!("../fonts/NotoSansSC-Regular.otf");
        const SMILEY: &[u8] = include_bytes!("../fonts/SmileySans-Oblique.ttf");
        const DOUYIN: &[u8] = include_bytes!("../fonts/DouyinSansBold.otf");
        const WENKAI: &[u8] = include_bytes!("../fonts/lxgw-wenkai-gb-lite-v1.501/LXGWWenKaiMonoGBLite-Medium.ttf");

        let mut fonts = egui::FontDefinitions::default();
        for (name, font) in [
            ("noto", NOTO),
            ("smiley", SMILEY),
            ("douyin", DOUYIN),
            ("wenkai", WENKAI),
        ] {
            fonts
                .font_data
                .insert(name.to_owned(), egui::FontData::from_static(font));

            let mut newfam = BTreeMap::new();
            newfam.insert(egui::FontFamily::Name(name.into()), vec![name.to_owned()]);
            fonts.families.append(&mut newfam);
        }

        cc.egui_ctx.set_fonts(fonts);
    }

    pub fn run_gui() {
        let native_options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size((800.0, 600.0)),
            ..eframe::NativeOptions::default()
        };
        eframe::run_native(
            "挂号游戏",
            native_options,
            Box::new(|cc| {
                cc.egui_ctx.set_zoom_factor(2.0);
                Self::add_fonts(cc);
                Ok(Box::new(Self::new()))
            }),
        )
        .unwrap();
    }
}
