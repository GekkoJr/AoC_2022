use std::fs;
use std::thread::{sleep};
use std::time;
use eframe::egui;
use egui::{Context, Color32, Pos2};

fn main() {
    let mut cords: Vec<i32> = vec![50, 50];


    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(9000.0, 1000.0)),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "Day9 AoC 2022",
        options,
        Box::new(|cc| {
            Box::<MyApp>::default()
        }),
    );
        

    struct MyApp {
        dots: Vec<(f32, f32)>,
        green_dots: Vec<bool>,
        speed: i32,
        line: i32,
        running: bool,
        all_lines: String,
        head: (f32, f32),
        tail: (f32, f32),
    }
    impl Default for MyApp {
        fn default() ->Self {

            let mut dots = Vec::new();
            let mut green_dots = Vec::new();
            let grid_size = 50;

            for x in 0..=grid_size {
                for y in 0..=grid_size {
                    dots.push((x as f32, y as f32));
                    green_dots.push(false);
                }
            }

            green_dots[find_pos(grid_size / 2, grid_size / 2)] = true;

            Self {
                dots,
                green_dots,
                speed: 100,
                line: 0,
                running: false,
                all_lines: fs::read_to_string("test.txt").unwrap(),
                head: (25.0, 25.0),
                tail: (25.0, 25.0)
            }
        }
    }


    impl eframe::App for MyApp {
        fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
            egui::SidePanel::left("side_panel").show(ctx, |ui| {
               ui.heading("Options")    ;
                ui.add(egui::Slider::new(&mut self.speed, 0..=100).text("Delay"));

                if ui.button("Run this shit").clicked() && (self.running == false) {
                    self.running = true
                }
                if self.running {
                    self.line += 1;
                    ui.label("Runnin line:");
                    ui.label(self.line.to_string());
                }
            });
            egui::CentralPanel::default().show(ctx, |ui| {
                for((i, (x, y))) in self.dots.iter().enumerate() {
                    let pos =  Pos2::new((*x * 10.0) + 250.0, (*y * 10.0) + 10.0);
                    let color = if self.green_dots[i] {
                        Color32::GREEN
                    } else {
                        Color32::GRAY
                    };

                    ui.painter().circle_filled(pos, 2.0, color)
                }
                if self.running {
                    sleep(time::Duration::from_millis((self.speed * 10) as u64));
                    Context::request_repaint(&ctx);
                }


            });
        }
    }
}

fn find_pos(x: i32, y: i32) -> usize {
    let mut result: i32 = 0;
    result = x + (51 * y);

    return result.to_string().parse::<usize>().unwrap();
}