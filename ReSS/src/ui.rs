use std::path::Path;
use std::process::Command;

use eframe::egui::{Color32, RichText, Ui};
use notify_rust::Notification;

use crate::{Layer, NAME, vec2};

/// The text color.
const TEXT_COLOR: Color32 = Color32::from_rgb(255, 255, 255);
/// Runtime executable.
const RUNTIME: &str = "PXStudioRuntimeMMO.exe";

/// Begins drawing the UI components.
///
/// # Arguments
///
/// * `layer`: The `Layer` struct which holds the components that are considered "dynamic" and can have their values changed.
/// * `ui`: Mutable reference to the `Ui` struct which is used to draw the components.
///
/// returns: ()
pub fn draw(layer: &mut Layer, ui: &mut Ui) {
    ui.vertical_centered(|centered| {
        centered.image(layer.raw_logo.texture_id(centered.ctx()), vec2(1024.0 - 600.0, 451.0 - 300.0));
        draw_heading(NAME, centered, TEXT_COLOR, 42.0);
        draw_heading("Enter the path to the old game files.", centered, TEXT_COLOR, 22.0);
        centered.text_edit_singleline(&mut layer.path);
        check_path(&layer.path, centered);
    });
}


/// Attempts to start a process.
///
/// # Arguments
///
/// * `directory`: Directory to automatically CD into.
/// * `file`: Executable file to run.
///
/// returns: () | If the spawn wasn't successful, returns an error in the console.
///
/// # Examples
///
/// ```
/// fn main() {
///    open_process("Z:\\Path", "File.exe");
/// }
/// ```
fn open_process(directory: &str, file: &str) {
    // Stupid "hack" of launching PXStudioRuntimeMMO since it explicitly requires us to cd into the directory and THEN run the executable.
    let mut child = Command::new("cmd").args(&["/c", "cd", directory, "&&", &(".\\".to_owned() + file)]).spawn().unwrap();
    // Pause the program until the child has closed or died.
    loop {
        match child.try_wait() {
            Ok(Some(exit)) => {
                println!("{} exited with status code: {}", file, exit);
                display_notification(RUNTIME, format!("Process exited with status code: {}", exit).as_str());
                break;
            }
            Ok(None) => continue,
            Err(e) => {
                println!("Error upon calling open_process: {}", e);
                break;
            }
        }
    }
}

/// Attempts to show a notification.
///
/// # Arguments
///
/// * `summary`: Summary of the notification.
/// * `text`: Text body of the notification.
///
/// returns: () - If the notification couldn't be shown, returns an error in the console.
///
/// # Examples
///
/// ```
/// fn main() {
///    display_notification("Summary", "Text Body");
/// }
/// ```
fn display_notification(summary: &str, text: &str) {
    match Notification::new().summary(summary).body(text).appname(NAME).show() {
        Ok(_) => (),
        Err(e) => println!("Error upon calling display_notification: {}", e),
    }
}

/// Expands upon the `[ ui.heading(*) ]` function to allow for more customization.
///
/// # Arguments
///
/// * `str`: The string to be displayed.
/// * `ui`: Mutable reference to the `Ui` struct which is used to draw the components.
/// * `color`: The color of the text.
/// * `font_size`: The size of the font.
///
/// returns: ()
fn draw_heading(str: &str, ui: &mut Ui, color: Color32, font_size: f32) { ui.label(RichText::new(str).color(color).size(font_size)); }

/// Draws components based on if the PXStudioRuntimeMMO.exe file exists in the specified directory or not.
///
/// # Arguments
///
/// * `path`: The path to check.
/// * `ui`: Mutable reference to the `Ui` struct which is used to draw the components.
///
/// returns: ()
fn check_path(path: &String, ui: &mut Ui) {
    if path == "" {
        ui.label("Waiting...");
        return;
    }
    if exists(path) {
        ui.label(format!("Path: '{}' contains '{}', ready to launch!", path, RUNTIME));
        if ui.button("Launch").clicked() {
            display_notification(RUNTIME, "Launching, please open POS.22.exe as soon as you see the loading screen!");
            open_process(path, RUNTIME);
        }
    } else {
        ui.label(format!("Path '{}' does not contain '{}', please try a different path!", path, RUNTIME));
    }
}

/// Checks if the path contains the file "PXStudioRuntimeMMO.exe".
///
/// # Arguments
///
/// * `path`: The path to check.
///
/// returns: bool
fn exists(path: &String) -> bool { Path::new(&path).join(RUNTIME).is_file() }