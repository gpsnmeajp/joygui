#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use egui::{FontDefinitions, FontData};
use windows::Win32::Media::Multimedia::*;
use std::env;

/*
https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Media/Multimedia/fn.joyGetPosEx.html
https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Media/Multimedia/struct.JOYINFOEX.html
*/

fn main() -> Result<(), eframe::Error> {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        centered: true,
        default_theme: eframe::Theme::Light,
        follow_system_theme: false,
        ..Default::default()
    };
    eframe::run_native("joygui", options, Box::new(|cc| Box::new(MyApp::new(cc))))
}

struct MyApp {}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            //default value
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint(); //毎フレーム更新を要求

        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert("my_font".to_owned(), FontData::from_static(include_bytes!("C:\\Windows\\Fonts\\yumin.ttf")));
        fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "my_font".to_owned());
        fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap().push("my_font".to_owned());
        ctx.set_fonts(fonts);


        let mut text = String::new();

        let nums: u32;
        unsafe {
            nums = joyGetNumDevs();
        }
        text += format!("デバイス数: {num}\n", num = nums).as_str();

        let mut joyinfoex: JOYINFOEX = JOYINFOEX::default();
        joyinfoex.dwSize = std::mem::size_of::<JOYINFOEX>() as u32;
        joyinfoex.dwFlags = (JOY_RETURNBUTTONS
            | JOY_RETURNCENTERED
            | JOY_RETURNPOV
            | JOY_RETURNR
            | JOY_RETURNU
            | JOY_RETURNV
            | JOY_RETURNX
            | JOY_RETURNY
            | JOY_RETURNZ) as u32;

        const ujoyid: u32 = JOYSTICKID1;
        let pji: *mut JOYINFOEX = &mut joyinfoex as *mut JOYINFOEX;
        text += format!("{:?}\n", pji).as_str();

        let ret: u32;
        unsafe {
            ret = joyGetPosEx(ujoyid, pji);
        }
        if ret == JOYERR_NOERROR {
            let dw_buttons = joyinfoex.dwButtons;
            let mut dw_xpos = joyinfoex.dwXpos;
            let mut dw_ypos = joyinfoex.dwYpos;
            let mut dw_zpos = joyinfoex.dwZpos;
            let mut dw_rpos = joyinfoex.dwRpos;
            let mut dw_upos = joyinfoex.dwUpos;
            let mut dw_vpos = joyinfoex.dwVpos;
            let mut dw_pov = joyinfoex.dwPOV;

            text += format!("{id} OK\n", id = ujoyid).as_str();
            text += format!("Buttons: {:?}\n", dw_buttons).as_str();
            text += format!("X: {:?}\n", dw_xpos).as_str();
            text += format!("Y: {:?}\n", dw_ypos).as_str();
            text += format!("Z: {:?}\n", dw_zpos).as_str();
            text += format!("R: {:?}\n", dw_rpos).as_str();
            text += format!("U: {:?}\n", dw_upos).as_str();
            text += format!("V: {:?}\n", dw_vpos).as_str();
            text += format!("POV: {:?}\n", dw_pov).as_str();

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading(text);
                ui.add(egui::Slider::new(&mut dw_xpos, 0..=65535).text("X"));
                ui.add(egui::Slider::new(&mut dw_ypos, 0..=65535).text("Y"));
                ui.add(egui::Slider::new(&mut dw_zpos, 0..=65535).text("Z"));
                ui.add(egui::Slider::new(&mut dw_rpos, 0..=65535).text("R"));
                ui.add(egui::Slider::new(&mut dw_upos, 0..=65535).text("U"));
                ui.add(egui::Slider::new(&mut dw_vpos, 0..=65535).text("V"));
                ui.add(egui::Slider::new(&mut dw_pov, 0..=36000).text("POV"));
            });
        } else {
            match ret {
                JOYERR_NOCANDO => text += format!("{id} JOYERR_NOCANDO\n", id = ujoyid).as_str(),
                JOYERR_PARMS => text += format!("{id} JOYERR_PARMS\n", id = ujoyid).as_str(),
                JOYERR_UNPLUGGED => text += format!("{id} JOYERR_UNPLUGGED\n", id = ujoyid).as_str(),
                _ => text += format!("{id} ?\n", id = ujoyid).as_str(),
            }

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading(text);
            });
        }

    }
}
