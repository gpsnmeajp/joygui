#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod joyget;

use eframe::egui;
use egui::{emath::RectTransform, FontData, FontDefinitions, Rounding};
use std::env;

/*
https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Media/Multimedia/fn.joyGetPosEx.html
https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Media/Multimedia/struct.JOYINFOEX.html

https://whoisryosuke.com/blog/2023/getting-started-with-egui-in-rust/
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
        fonts.font_data.insert(
            "my_font".to_owned(),
            FontData::from_static(include_bytes!("C:\\Windows\\Fonts\\yumin.ttf")),
        );
        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "my_font".to_owned());
        fonts
            .families
            .get_mut(&egui::FontFamily::Monospace)
            .unwrap()
            .push("my_font".to_owned());
        ctx.set_fonts(fonts);

        let gamepad = joyget::update(0);

        match gamepad {
            Ok(d) => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let (responce, painter) =
                        ui.allocate_painter(egui::Vec2::new(100.0, 100.0), egui::Sense::hover());
                    let to_screen = RectTransform::from_to(
                        egui::Rect::from_min_size(egui::Pos2::ZERO, responce.rect.size()),
                        responce.rect,
                    );
                    let p1 = to_screen.transform_pos(egui::Pos2 { x: 0.0, y: 0.0 });
                    let p2 = to_screen.transform_pos(egui::Pos2 { x: 100.0, y: 100.0 });
                    painter.add(egui::Shape::Rect(egui::epaint::RectShape {
                        rect: egui::Rect { min: p1, max: p2 },
                        rounding: Rounding::none(),
                        fill: egui::Color32::WHITE,
                        stroke: egui::Stroke {
                            width: 1.0,
                            color: egui::Color32::BLACK,
                        },
                    }));

                    let pos_x = ((d.axis_x + 1.0) / 2.0) * 100.0;
                    let pos_y = ((d.axis_y + 1.0) / 2.0) * 100.0;

                    let q1 = to_screen.transform_pos(egui::Pos2 {
                        x: pos_x,
                        y: pos_y - 5.0,
                    });
                    let q2 = to_screen.transform_pos(egui::Pos2 {
                        x: pos_x,
                        y: pos_y + 5.0,
                    });
                    painter.add(egui::Shape::LineSegment {
                        points: [q1, q2],
                        stroke: egui::Stroke {
                            width: 1.0,
                            color: egui::Color32::RED,
                        },
                    });

                    let q1 = to_screen.transform_pos(egui::Pos2 {
                        x: pos_x - 5.0,
                        y: pos_y,
                    });
                    let q2 = to_screen.transform_pos(egui::Pos2 {
                        x: pos_x + 5.0,
                        y: pos_y,
                    });
                    painter.add(egui::Shape::LineSegment {
                        points: [q1, q2],
                        stroke: egui::Stroke {
                            width: 1.0,
                            color: egui::Color32::RED,
                        },
                    });

                    ui.add(egui::Slider::new(&mut d.axis_x.to_owned(), -1.0..=1.0).text("X"));
                    ui.add(egui::Slider::new(&mut d.axis_y.to_owned(), -1.0..=1.0).text("Y"));
                    ui.add(egui::Slider::new(&mut d.axis_z.to_owned(), -1.0..=1.0).text("Z"));
                    ui.add(egui::Slider::new(&mut d.axis_r.to_owned(), -1.0..=1.0).text("R"));
                    ui.add(egui::Slider::new(&mut d.axis_u.to_owned(), -1.0..=1.0).text("U"));
                    ui.add(egui::Slider::new(&mut d.axis_v.to_owned(), -1.0..=1.0).text("V"));
                    ui.add(
                        egui::Slider::new(&mut d.pov.unwrap_or_else(|| 65535), 0..=36000)
                            .text("POV"),
                    );

                    ui.horizontal(|ui| {
                        for i in 0..16 {
                            if d.buttons[i] {
                                ui.add_sized(
                                    [20.0, 20.0],
                                    egui::Label::new(
                                        egui::RichText::new(format!(" {} ", i + 1))
                                            .monospace()
                                            .background_color(egui::Color32::from_rgb(255, 0, 0))
                                            .color(egui::Color32::from_rgb(255, 255, 255)),
                                    ),
                                );
                            } else {
                                ui.add_sized(
                                    [20.0, 20.0],
                                    egui::Label::new(
                                        egui::RichText::new(format!(" {} ", i + 1))
                                            .monospace()
                                            .background_color(egui::Color32::from_rgb(
                                                127, 127, 127,
                                            ))
                                            .color(egui::Color32::from_rgb(0, 0, 0)),
                                    ),
                                );
                            }
                        }
                    });
                });
            }
            Err(e) => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading(format!("{:?}", e));
                });
            }
        }
    }
}
