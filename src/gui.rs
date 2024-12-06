use crate::{
    basic::{Action, Resource, RoundOutcome, INIT_STATE},
    player::BotPlayer,
};
use eframe::egui;
use std::collections::BTreeMap;

pub struct GUIApp {
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
        egui::TopBottomPanel::bottom("button_panel")
            .frame(egui::Frame {
                inner_margin: egui::Margin::symmetric(8.0, 8.0),
                fill: egui::Visuals::dark().panel_fill,
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
                    self.show_outcome(ui);
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
    fn update_state(&mut self) {
        let action = self.action.unwrap();
        let other_action = self.other_action.unwrap();
        for (s, c) in self.state.iter_mut().zip(action.get_cost().into_iter()) {
            *s -= c;
            if *s < 0 {
                self.outcome = RoundOutcome::Lose;
                return;
            }
        }

        for (s, c) in self.other_state.iter_mut().zip(other_action.get_cost().into_iter()) {
            *s -= c;
            if *s < 0 {
                self.outcome = RoundOutcome::Win;
                return;
            }
        }

        self.outcome = if let Action::Attack(a1) = action {
            match other_action {
                Action::Attack(a2) if a2 < a1 => RoundOutcome::Win,
                Action::Attack(a2) if a2 == a1 => RoundOutcome::Continue,
                Action::Attack(_) /* a2 > a1 */ => RoundOutcome::Lose,
                Action::Defend(d2) if d2 == a1 => RoundOutcome::Continue,
                Action::Defend(_) => RoundOutcome::Win,
                Action::Guahao => RoundOutcome::Win,
                Action::Quanfang => RoundOutcome::Continue,
                Action::Fantan => RoundOutcome::Lose,
            }
        } else if let Action::Attack(a2) = other_action {
            match action {
                Action::Attack(_) => unreachable!(),
                Action::Defend(d1) if d1 == a2 => RoundOutcome::Continue,
                Action::Defend(_) => RoundOutcome::Lose,
                Action::Guahao => RoundOutcome::Lose,
                Action::Quanfang => RoundOutcome::Continue,
                Action::Fantan => RoundOutcome::Win,
            }
        } else {
            RoundOutcome::Continue
        }
    }

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

    fn get_action_color(action: Action) -> egui::Color32 {
        match action {
            Action::Guahao => egui::Color32::from_rgb(135, 206, 235), // Sky blue
            Action::Attack(_) => egui::Color32::LIGHT_RED,
            Action::Defend(_) => egui::Color32::LIGHT_GREEN,
            Action::Fantan | Action::Quanfang => egui::Color32::ORANGE,
        }
    }

    fn create_text(text: &str, family: &str, size: f32) -> egui::RichText {
        egui::RichText::new(text)
            .family(egui::FontFamily::Name(family.into()))
            .size(size)
    }
}

impl GUIApp {
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
                self.action = Some(action);
                self.other_action = Some(BotPlayer::get_action(self.other_state, self.state));
                self.update_state();
                self.update_legal_actions();
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

    fn show_outcome(&mut self, ui: &mut egui::Ui) {
        self.is_active = false;
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
}

impl GUIApp {
    fn new() -> Self {
        let mut gui_app = Self {
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
        const WENKAI: &[u8] = include_bytes!("../fonts/lxgw-wenkai-gb-lite-v1.501/LXGWWenKaiMonoGBLite-Medium.ttf");

        let mut fonts = egui::FontDefinitions::default();
        for (name, font) in [("noto", NOTO), ("smiley", SMILEY), ("wenkai", WENKAI)] {
            fonts
                .font_data
                .insert(name.to_owned(), egui::FontData::from_static(font));

            let mut newfam = BTreeMap::new();
            newfam.insert(egui::FontFamily::Name(name.into()), vec![name.to_owned()]);
            fonts.families.append(&mut newfam);
        }

        cc.egui_ctx.set_fonts(fonts);
    }

    pub fn create_app(
        cc: &eframe::CreationContext<'_>,
    ) -> Result<Box<dyn eframe::App>, Box<dyn std::error::Error + Send + Sync>> {
        cc.egui_ctx.set_zoom_factor(2.0);
        cc.egui_ctx.set_theme(egui::Theme::Dark);
        Self::add_fonts(cc);
        Ok(Box::new(Self::new()))
    }
}
