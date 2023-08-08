use bevy::prelude::ResMut;
use bevy_egui::{
    egui::{self, Color32, RichText, Context},
    EguiContexts,
};

use crate::utils::{self, state::State};

fn display_offline_form(ctx: &Context, state: &mut State) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.group(|ui| {
            ui.vertical_centered(|ui| {
                ui.heading(RichText::new("Offline connection").size(40.0));

                ui.add_space(12.0);

                ui.label(RichText::new("Enter your username").size(18.0));
                ui.add(
                    egui::TextEdit::singleline(&mut state.user.username)
                        .min_size([100.0, 45.0].into())
                        .password(true),
                );

                ui.add_space(5.0);

                if state.user.username.is_empty() {
                    ui.label(
                        RichText::new("You must provide a valid username...")
                            .size(18.0)
                            .color(Color32::LIGHT_RED),
                    );
                } else if ui.button(RichText::new("Connect").size(18.0)).clicked() {
                    println!("Connecting with username: {}", state.user.username);
                }

                ui.add_space(5.0);

                if ui.button(RichText::new("Switch to online").size(18.0)).clicked()
                {
                    state.toggle_session();
                }
            });
        });
    });
}

fn display_online_form(ctx: &Context, state: &mut State) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.set_max_size([200.0, 100.0].into());

        ui.group(|ui| {
            ui.vertical_centered(|ui| {
                ui.heading(RichText::new("Online connection").size(40.0));

                ui.add_space(12.0);

                ui.label(RichText::new("Enter your username").size(18.0));
                ui.add(
                    egui::TextEdit::singleline(&mut state.user.username).min_size([100.0, 45.0].into()),
                );

                ui.add_space(5.0);

                ui.label(RichText::new("Enter your password").size(18.0));
                ui.add(
                    egui::TextEdit::singleline(&mut state.user.password)
                        .min_size([100.0, 45.0].into())
                        .password(true),
                );

                ui.add_space(5.0);

                if state.user.username.is_empty() || state.user.password.is_empty() {
                    ui.label(
                        RichText::new("You must provide a valid username and password...")
                            .size(18.0)
                            .color(Color32::LIGHT_RED),
                    );
                } else {
                    if ui.button(RichText::new("Connect").size(18.0)).clicked()
                    {
                        println!("Connecting with username: {}", state.user.username);
                    }
                }

                ui.add_space(5.0);

                if ui.button(RichText::new("Switch to offline").size(18.0)).clicked()
                {
                    state.toggle_session();
                }
            });
        });
    });
}

fn display_toolbar(ctx: &Context) {
    egui::TopBottomPanel::top("Toolbar").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            if ui.button(RichText::new("Files").size(18.0)).clicked() {
                
            }
            if ui.button(RichText::new("About").size(18.0)).clicked() {
                ui.vertical_centered(|ui| {
                    ui.label("over");
                    ui.label("under");
                });
            }
        });
    });
}

pub fn display_form(mut ctxs: EguiContexts, mut state: ResMut<utils::state::State>) {
    let ctx = ctxs.ctx_mut();
    display_toolbar(ctx);

    if state.is_offline() {
        display_offline_form(ctx, &mut state);
    } else {
        display_online_form(ctx, &mut state);
    }
}
