use std::fs;
use std::thread::{sleep};
use std::time;
use eframe::egui;
use egui::{Context, Color32, Pos2, Stroke};

fn main() {
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
        line: usize,
        running: bool,
        all_lines: String,
        head: (f32, f32),
        tail: (f32, f32),
        part: i32,
        total_lines: usize,
        total: i32,
    }
    impl Default for MyApp {
        fn default() ->Self {

            let all_lines  = fs::read_to_string("test.txt").unwrap();
            let mut dots = Vec::new();
            let mut green_dots = Vec::new();
            let grid_size = 50;

            for x in 0..=grid_size {
                for y in 0..=grid_size {
                    dots.push((x as f32, y as f32));
                    green_dots.push(false);
                }
            }

            let mut total = 0;
            for _i in all_lines.lines() {
                total += 1;
            }
            let total_lines = total;

            green_dots[find_pos(grid_size / 2, grid_size / 2)] = true;

            Self {
                dots,
                green_dots,
                speed: 100,
                line: 0,
                part: 0,
                running: false,
                all_lines,
                head: (25.0, 25.0),
                tail: (25.0, 25.0),
                total_lines,
                total : 0,
            }
        }
    }


    impl eframe::App for MyApp {
        fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
            egui::SidePanel::left("side_panel").show(ctx, |ui| {
               ui.heading("Options")    ;
                ui.add(egui::Slider::new(&mut self.speed, 0..=100).text("Delay"));

                if ui.button("Run this shit").clicked() && (self.running == false) {
                    self.running = true;
                    self.head = (25.0, 25.0);
                    self.tail = (25.0, 25.0);
                    self.total = 0;
                }
                if self.running {
                    ui.label("Runnin line:");
                    ui.label(self.line.to_string());
                }
                ui.label(format!("Total {}", self.total))
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
                // painting head
                let (hx, hy) = self.head;
                let head_pos: Pos2 = Pos2::new((hx * 10.0) + 250.0, (hy * 10.0) + 10.0);
                ui.painter().circle(head_pos, 3.0, Color32::TRANSPARENT, Stroke::new(1.0, Color32::RED ));

                // painting tail
                let (tx, ty) = self.tail;
                let tail_pos: Pos2 = Pos2::new((tx * 10.0) + 250.0, (ty * 10.0) + 10.0);
                ui.painter().circle(tail_pos, 3.0, Color32::TRANSPARENT, Stroke::new(0.5, Color32::YELLOW));

                // painting arrow
                ui.painter().arrow(
                    tail_pos,
                    head_pos - tail_pos,
                    Stroke::new(1.0, Color32::YELLOW)
                );

                if self.running {

                    let line: Vec<&str> = self.all_lines.lines().collect::<Vec<&str>>()[self.line].split(" ").collect();
                    let direction = line[0];
                    let steps = line[1];

                    // do 1 instruction
                    if self.part < steps.parse::<i32>().unwrap() {
                        // moving head
                        let (mut hx ,mut hy) = self.head;
                        println!("{hx}-{hy}");
                        match direction {
                            "D" => hy -= 1.0,
                            "U" => hy += 1.0,
                            "L" => hx -= 1.0,
                            "R" => hx += 1.0,
                            _ => panic!("Invild instruct")
                        };
                        self.head = (hx, hy);
                        println!("Head moved to {hx}-{hy} Steps {steps} Part of {}", self.part);
                        self.part +=1;

                        // move tail
                        let (mut tx, mut ty) = self.tail;
                        let x_diff = hx - tx;
                        let y_diff = hy - ty;
                        println!("Diff is x {x_diff}");

                        // makes sure it only moves one field


                        let mut moved = false;
                        if (x_diff == 2.0) && !moved {
                            if y_diff != 0.0 {
                                ty += y_diff;
                            }
                            tx += 1.0;
                            moved == true;
                        }
                        if x_diff == -2.0 {
                            if y_diff != 0.0 {
                                ty += y_diff;
                            }
                            tx -= 1.0;
                            moved == true;
                        }
                        if (y_diff == 2.0) && !moved {
                            if x_diff != 0.0 {
                                tx += x_diff;
                            }
                            ty += 1.0;
                            moved == true;
                        }

                        if (y_diff == -2.0) && !moved {
                            if x_diff != 0.0 {
                                tx += x_diff;
                            }
                            ty -= 1.0;
                            moved == true;
                        }


                        self.tail = (tx, ty);
                        self.green_dots[find_pos(tx as i32, ty as i32)] = true;

                    } else {
                        self.part = 0;
                        self.line += 1;
                    }

                    if self.line == self.total_lines {
                        self.running = false;
                        self.line = 0;
                        Context::request_repaint(&ctx);

                        for i in self.green_dots.clone() {
                            if i == true {
                                self.total += 1;
                            }
                        }

                    }

                    sleep(time::Duration::from_millis((self.speed * 10) as u64));
                    Context::request_repaint(&ctx);
                }


            });
        }
    }
}

fn find_pos(x: i32, y: i32) -> usize {
    let mut result: i32 = 0;
    result = y + (51 * x);

    return result.to_string().parse::<usize>().unwrap();
}