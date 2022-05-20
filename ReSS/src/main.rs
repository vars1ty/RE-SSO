#![allow(non_snake_case)]

use eframe::{App, Frame, NativeOptions, run_native};
use eframe::egui::{CentralPanel, Context, vec2, Visuals};
use egui_extras::RetainedImage;

mod ui;

/**
 * RE:[SSO] - Launcher
 * This is made for educational and learning purposes only, I do not represent "Star Stable Entertainment AB" in any way, shape or form.
 * Use this at your own risk, no warranty is given and it is not guaranteed to work for everyone.
 * Help will only be given as long as you aren't using this for any malicious purposes, have read the license AND have a basic understanding of how this works.
 */

/// Application name.
pub const NAME: &str = "RE:[SSO] Launcher";

/// Startup function.
fn main() {
    let size = Option::from(vec2(520.0, 320.0));
    let op = NativeOptions {
        always_on_top: false,
        maximized: false,
        decorated: true,
        drag_and_drop_support: false,
        icon_data: None,
        initial_window_pos: None,
        initial_window_size: size,
        min_window_size: size,
        max_window_size: size,
        resizable: false,
        transparent: false,
        vsync: true,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
    };
    run_native(NAME, op, Box::new(|_| Box::new(Layer::default())));
}

/// A simple structure which holds the components that are considered "dynamic" and can have their values changed.
pub struct Layer {
    pub path: String,
    pub raw_logo: RetainedImage,
}

/// Default values for the Layer struct.
impl Default for Layer {
    fn default() -> Self {
        Self {
            path: "".to_string(),
            raw_logo: RetainedImage::from_image_bytes("../assets/logo.png", include_bytes!("../assets/logo.png")).unwrap(),
        }
    }
}

/// Update loop for the application, responsible for calling other classes which then draw the components.
impl App for Layer {
    /// Updates the UI.
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |_ui| {
            ui::draw(self, _ui);
            ctx.set_visuals(Visuals::dark());
        });
    }
}