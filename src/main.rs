use std::fmt::Display;

use eframe::{egui, epaint::Fonts};
use egui::*;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("calc", native_options, Box::new(|cc| Box::new(CalcApp::new(cc)))).unwrap();
}

#[derive(PartialEq)]
enum Photographer{
    Ken,
    Colin,
    Team,
}

impl Display for Photographer{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Photographer::Ken => write!(f,"Ken"),
            Photographer::Colin => write!(f,"Colin"),
            Photographer::Team => write!(f, "Team"),
        }
    }
}

impl Photographer {
    fn get_hourly(&self) -> f32{
        match self{
            Photographer::Ken => 275.0,
            Photographer::Colin => 200.0,
            Photographer::Team => 150.0,
        }
    }
    fn get_first_half_day(&self) -> f32{
        match self{
            Photographer::Ken => 1500.0,
            Photographer::Colin => 1000.0,
            Photographer::Team => 600.0,
        }
    }
    fn get_second_half_day(&self) -> f32{
        match self{
            Photographer::Ken => 1000.0,
            Photographer::Colin => 800.0,
            Photographer::Team => 600.0,
        }
    }
}

enum ShootType{
    Hourly{
        hours: f32,
        image_prep: bool,
    },
    HalfDayBased{
        halves: u32,
        image_prep: bool,
    },
    Headshot{
        hours: f32,
        people: u32,
        editing: bool,
        retouch: RetouchLevel,
    },
}

impl Display for ShootType{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            ShootType::Hourly {..} => write!(f,"Hourly"),
            ShootType::HalfDayBased {..} => write!(f,"Half Day"),
            ShootType::Headshot {..} => write!(f,"Headshot"),
        }
    }
}

impl PartialEq for ShootType{
    fn eq(&self, other: &Self) -> bool {
        match (self, other){
            (ShootType::Hourly {..}, ShootType::Hourly {..}) |
            (ShootType::Headshot {..}, ShootType::Headshot {..}) |
            (ShootType::HalfDayBased {..}, ShootType::HalfDayBased {..}) => true,
            _ => false,
        }
    }
}

enum RetouchLevel{
    Volume,
    Light,
    Nice,
}

impl RetouchLevel {
    fn get_price_per(&self) -> f32{
        match self{
            RetouchLevel::Volume => 5.0,
            RetouchLevel::Light => 10.0,
            RetouchLevel::Nice => 20.0,
        }
    }
}

impl Display for RetouchLevel{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            RetouchLevel::Volume => write!(f, "Volume"),
            RetouchLevel::Light => write!(f, "Light"),
            RetouchLevel::Nice => write!(f, "Nice"),
        }
    }
}

impl PartialEq for RetouchLevel{
    fn eq(&self, other: &Self) -> bool {
        match (self, other){
            (RetouchLevel::Volume, RetouchLevel::Volume) |
            (RetouchLevel::Light, RetouchLevel::Light) |
            (RetouchLevel::Nice, RetouchLevel::Nice) => true,
            _ => false,
        }
    }
}

const IMAGE_PREP_PRICE: f32 = 50.0;
const DRONE_PRICE: f32 = 150.0;
const ASSISTANT_PRICE: f32 = 40.0;
const EXPENSES_PRICE: f32 = 10.0;
const ON_SITE_EDITIING_PRICE: f32 = 100.0;

struct CalcApp{
    photographer: Photographer,
    shoot_type: ShootType,
    assistant_hours: f32,
    expenses: u32,
    drone: bool,
}

impl CalcApp{
    fn new(_cc: &eframe::CreationContext<'_>) -> CalcApp{
        CalcApp {
            photographer: Photographer::Ken,
            shoot_type: ShootType::Hourly { hours: 0.0, image_prep: false },
            assistant_hours: 0.0,
            expenses: 0,
            drone: false,
        }
    }

    fn calc_price(&self) -> f32{
        match &self.shoot_type{
            ShootType::Hourly { hours, image_prep } => {
                (self.photographer.get_hourly() * hours) 
                + if *image_prep {IMAGE_PREP_PRICE as f32} else {0.0} + if self.drone {DRONE_PRICE as f32} else {0.0}
                + self.assistant_hours as f32 * ASSISTANT_PRICE as f32
                + self.expenses as f32 * EXPENSES_PRICE as f32
            },
            ShootType::HalfDayBased { halves, image_prep } => {
                ((*halves as f32/2.0).ceil() * self.photographer.get_first_half_day())
                + ((*halves as f32/2.0).floor() * self.photographer.get_second_half_day())
                + if *image_prep {IMAGE_PREP_PRICE} else {0.0} + if self.drone {DRONE_PRICE} else {0.0}
                + self.assistant_hours * ASSISTANT_PRICE
                + self.expenses as f32 * EXPENSES_PRICE
            },
            ShootType::Headshot { hours, people, editing, retouch } => {
                (self.photographer.get_hourly() * hours) 
                + if self.drone {DRONE_PRICE} else {0.0}
                + self.assistant_hours * ASSISTANT_PRICE
                + self.expenses as f32 * EXPENSES_PRICE
                + *people as f32 * retouch.get_price_per()
                + if *editing {ON_SITE_EDITIING_PRICE} else {0.0}
            },
        }
    }
}

impl eframe::App for CalcApp{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(4.0);
        egui::CentralPanel::default().show(ctx,|ui|{
            egui::ComboBox::from_label("Shoot Type")
            .selected_text(&self.shoot_type.to_string())
            .show_ui(ui, |ui|{
                ui.selectable_value(&mut self.shoot_type, ShootType::Hourly { hours: 0.0, image_prep: false}, "Hourly");
                ui.selectable_value(&mut self.shoot_type, ShootType::HalfDayBased { halves: 0, image_prep: false}, "Half Day");
                ui.selectable_value(&mut self.shoot_type, ShootType::Headshot { hours: 0.0, people: 0, editing: false, retouch: RetouchLevel::Volume}, "Headshot");
            });
            egui::ComboBox::from_label("Photographer")
            .selected_text(&self.photographer.to_string())
            .show_ui(ui, |ui|{
                ui.selectable_value(&mut self.photographer, Photographer::Ken, "Ken");
                ui.selectable_value(&mut self.photographer, Photographer::Colin, "Colin");
                ui.selectable_value(&mut self.photographer, Photographer::Team, "Team");
            });
            ui.horizontal(|ui|{
                ui.add(DragValue::new(&mut self.expenses));
                ui.label("expenses (10$ per)");
            });
            ui.checkbox(&mut self.drone, "drone ($150)");
            match &mut self.shoot_type{
                ShootType::Hourly { hours, image_prep } =>{
                    ui.horizontal(|ui|{
                        ui.add(DragValue::new(hours));
                        if *hours == 0.0{
                            ui.label("hour");
                        } else{
                            ui.label("hours");
                        }
                    });

                    ui.checkbox(image_prep, "image prep");
                },

                ShootType::Headshot { hours, people, editing, retouch } => {
                    ui.horizontal(|ui|{
                        ui.add(DragValue::new(hours));
                        if *hours == 1.0{
                            ui.label("hour");
                        } else{
                            ui.label("hours");
                        }
                    });

                    ui.horizontal(|ui|{
                        ui.add(DragValue::new(people));
                        if *hours == 1.0{
                            ui.label("person");
                        } else{
                            ui.label("people");
                        }
                    });

                    egui::ComboBox::from_label("Retouching Type")
                    .selected_text(retouch.to_string())
                    .show_ui(ui, |ui|{
                        ui.selectable_value(retouch, RetouchLevel::Volume, "Volume");
                        ui.selectable_value(retouch, RetouchLevel::Light, "Light");
                        ui.selectable_value(retouch, RetouchLevel::Nice, "Nice");
                    });

                    ui.checkbox(editing, "editing");
                }

                ShootType::HalfDayBased { halves, image_prep } =>{
                    ui.horizontal(|ui|{
                        ui.add(DragValue::new(halves));
                        ui.label("number of half days");
                    });
                    
                    ui.checkbox(image_prep, "image prep");
                }
            }
            if ui.add(Label::new(self.calc_price().to_string()).sense(Sense::click())).on_hover_text("click to copy").clicked(){
                ui.output_mut(|o|{ o.copied_text = self.calc_price().to_string()});
            };
        });
    }
}
