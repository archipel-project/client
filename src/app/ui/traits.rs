use bevy_egui::egui::Context;

pub trait EguiWidget {
    fn show(&mut self, ctx: &Context);
}
