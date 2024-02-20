use eframe::egui;
use egui::*;

mod calc;

use calc::*;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "calc",
        native_options,
        Box::new(|cc| Box::new(CalcApp::new(cc))),
    )
    .unwrap();
}

impl eframe::App for CalcApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.set_pixels_per_point(frame.info().native_pixels_per_point.unwrap_or(1.0) * 2.0);

            egui::ComboBox::from_label("Shoot Type")
                .selected_text(self.shoot_type.to_string())
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.shoot_type,
                        ShootType::Hourly {
                            hours: 0.0,
                            image_prep: false,
                            assistant_hours: 0.0,
                            use_higher_prep_price: false,
                            use_higher_assistant_price: false,
                            photographer: Photographer::Ken,
                        },
                        "Hourly",
                    );
                    ui.selectable_value(
                        &mut self.shoot_type,
                        ShootType::HalfDayBased {
                            halves: 0,
                            image_prep: false,
                            assistant_hours: 0.0,
                            use_higher_prep_price: false,
                            use_higher_assistant_price: false,
                            photographer: Photographer::Ken,
                        },
                        "Half Day",
                    );
                    ui.selectable_value(
                        &mut self.shoot_type,
                        ShootType::Headshot {
                            heads: 0,
                            headshot_type: HeadshotType::Large,
                            editing: false,
                            retouch_level: RetouchLevel::Discount,
                            extra_retouched_photos: 0,
                            days: 1,
                        },
                        "Headshot",
                    );
                    ui.selectable_value(
                        &mut self.shoot_type,
                        ShootType::Conference { hours: 0.0 },
                        "Conference",
                    );
                });
            ui.horizontal(|ui| {
                ui.add(DragValue::new(&mut self.expenses));
                ui.label("expenses (10$ per)");
            });

            ui.checkbox(&mut self.drone, "drone ($150)");
            match &mut self.shoot_type {
                ShootType::Hourly {
                    hours,
                    image_prep,
                    assistant_hours,
                    photographer,
                    use_higher_assistant_price,
                    use_higher_prep_price,
                } => {
                    ui_hourly(ui, hours, image_prep, assistant_hours, photographer, use_higher_prep_price, use_higher_assistant_price);
                }

                ShootType::HalfDayBased {
                    halves,
                    image_prep,
                    assistant_hours,
                    photographer,
                    use_higher_assistant_price,
                    use_higher_prep_price,
                } => {
                    ui_half_day_based(ui, halves, image_prep, assistant_hours, photographer, use_higher_prep_price, use_higher_assistant_price);
                }

                ShootType::Headshot {
                    heads,
                    headshot_type,
                    editing,
                    retouch_level,
                    extra_retouched_photos,
                    days,
                } => {
                    ui_headshot(ui, heads, headshot_type, editing, retouch_level, extra_retouched_photos, days);
                }
                ShootType::Conference { hours } => {
                    ui_conference(ui, hours)
                }
            }
            if ui
                .add(
                    Label::new(format!("${}", self.calc_price().to_string())).sense(Sense::click()),
                )
                .on_hover_text("click to copy")
                .clicked()
            {
                ui.output_mut(|o| o.copied_text = self.calc_price().to_string());
            };
        });
    }
}

fn ui_hourly(
    ui: &mut Ui,
    hours: &mut f32,
    image_prep: &mut bool,
    assistant_hours: &mut f32,
    photographer: &mut Photographer,
    use_higher_prep_price: &mut bool, 
    use_higher_assistant_price: &mut bool,
) {
    egui::ComboBox::from_label("Photographer")
        .selected_text(photographer.to_string())
        .show_ui(ui, |ui| {
            ui.selectable_value(photographer, Photographer::Ken, "Ken");
            ui.selectable_value(photographer, Photographer::Colin, "Colin");
            ui.selectable_value(photographer, Photographer::Team, "Team");
        });
    ui.horizontal(|ui| {
        ui.add(DragValue::new(hours));
        if *hours == 0.0 {
            ui.label("hour");
        } else {
            ui.label("hours");
        }
    });

    ui.horizontal(|ui| {
        ui.add(DragValue::new(assistant_hours));
        if *assistant_hours == 1.0 {
            ui.label("assistant hour");
        } else {
            ui.label("assistant hours");
        }
        ui.checkbox(use_higher_assistant_price, "use higher assistant price")
    });

    ui.checkbox(image_prep, "image prep");
    ui.checkbox(use_higher_prep_price, "use higher image prep price");
}

fn ui_half_day_based(
    ui: &mut Ui,
    halves: &mut u32,
    image_prep: &mut bool,
    assistant_hours: &mut f32,
    photographer: &mut Photographer,
    use_higher_prep_price: &mut bool, 
    use_higher_assistant_price: &mut bool,
) {
    egui::ComboBox::from_label("Photographer")
        .selected_text(photographer.to_string())
        .show_ui(ui, |ui| {
            ui.selectable_value(photographer, Photographer::Ken, "Ken");
            ui.selectable_value(photographer, Photographer::Colin, "Colin");
            ui.selectable_value(photographer, Photographer::Team, "Team");
        });
    ui.horizontal(|ui| {
        ui.add(DragValue::new(halves));
        ui.label("number of half days");
    });

    ui.horizontal(|ui| {
        ui.add(DragValue::new(assistant_hours));
        if *assistant_hours == 1.0 {
            ui.label("assistant hour");
        } else {
            ui.label("assistant hours");
        }
        ui.checkbox(use_higher_assistant_price, "use higher assistant price")
    });

    ui.checkbox(image_prep, "image prep");
    ui.checkbox(use_higher_prep_price, "use higher image prep price");
}

fn ui_headshot(
    ui: &mut Ui,
    heads: &mut u32,
    headshot_type: &mut HeadshotType,
    editing: &mut bool,
    retouch_level: &mut RetouchLevel,
    extra_retouched_photos: &mut u32,
    days: &mut u32,
) {
    egui::ComboBox::from_label("Headshot type")
        .selected_text(headshot_type.to_string())
        .show_ui(ui, |ui| {
            ui.selectable_value(headshot_type, HeadshotType::Large, "Large");
            ui.selectable_value(headshot_type, HeadshotType::Team, "Team");
            ui.selectable_value(headshot_type, HeadshotType::Small, "Small");
        });
    
    egui::ComboBox::from_label("Retouching type")
        .selected_text(retouch_level.to_string())
        .show_ui(ui, |ui| {
            ui.selectable_value(retouch_level, RetouchLevel::Student, RetouchLevel::Student.to_string());
            ui.selectable_value(retouch_level, RetouchLevel::Discount, RetouchLevel::Discount.to_string());
            ui.selectable_value(retouch_level, RetouchLevel::Corporate, RetouchLevel::Corporate.to_string());
            ui.selectable_value(retouch_level, RetouchLevel::Full, RetouchLevel::Full.to_string());
        });

    ui.horizontal(|ui| {
        ui.add(DragValue::new(days));
        if *days == 1 {
            ui.label("day");
        } else {
            ui.label("days");
        }
    });

    ui.horizontal(|ui| {
        ui.add(DragValue::new(heads));
        if *heads == 1 {
            ui.label("person");
        } else {
            ui.label("people");
        }
    });
    
    ui.horizontal(|ui| {
        ui.add(DragValue::new(extra_retouched_photos));
        if *extra_retouched_photos == 1 {
            ui.label("additional retouched photo");
        } else {
            ui.label("additional retouched photos");
        }
    });

    ui.checkbox(editing, "on site editing");

    //extra text
    ui.separator();
    match headshot_type {
        HeadshotType::Large => {
            let text = format!(
"Features:
nice retouching included
on-site photo choice on iPad
12/people per hour
for up to {} photo hours (plus one extra hour on site for set-up & teardown)",
                calc_hours(*heads) - 1.0
            );
            if ui
                .add(Label::new(text.clone()).sense(Sense::click()))
                .on_hover_text("click to copy")
                .clicked()
            {
                ui.output_mut(|o| o.copied_text = text.to_string());
            };
        }
        HeadshotType::Team => {
            let text = 
"Features:
business-level retouching included (blemishes, flyaway hair
on-site photo choice on iPad
Online sign up & direct email delivery to subjects
12/people per hour
for up to 1 photo hour (plus one extra hour on site for set-up & teardown)";
            if ui
                .add(Label::new(text).sense(Sense::click()))
                .on_hover_text("click to copy")
                .clicked()
            {
                ui.output_mut(|o| o.copied_text = text.to_string());
            };
        }
        HeadshotType::Small => {
            let text = 
"Features:
simple lighting as needed
minor retouching included (blemishes)
fully trained HuthPhoto Team photographer";
            if ui
                .add(Label::new(text).sense(Sense::click()))
                .on_hover_text("click to copy")
                .clicked()
            {
                ui.output_mut(|o| o.copied_text = text.to_string());
            };
        }
    }
}

fn ui_conference(
    ui: &mut Ui,
    hours: &mut f32,
) {
    ui.horizontal(|ui| {
        ui.add(DragValue::new(hours));
        if *hours == 0.0 {
            ui.label("hour");
        } else {
            ui.label("hours");
        }
    });
}
