use core::fmt::Display;

const IMAGE_PREP_PRICE: f32 = 50.0;
const DRONE_PRICE: f32 = 150.0;
const ASSISTANT_PRICE: f32 = 40.0;
const EXPENSES_PRICE: f32 = 10.0;
const ON_SITE_EDITIING_PRICE: f32 = 100.0;

const LARGE_HEADSHOT_HOURLY: f32 = 275.0;
const TEAM_HEADSHOT_HOURLY: f32 = 200.0;

const CHEAP_RETOUCH_PRICE: f32 = 10.0;
const FANCY_RETOUCH_PRICE: f32 = 20.0;

pub struct CalcApp{
    pub shoot_type: ShootType,
    pub expenses: u32,
    pub drone: bool,
}

impl CalcApp{
    pub fn new(_cc: &eframe::CreationContext<'_>) -> CalcApp{
        CalcApp {
            shoot_type: ShootType::Hourly { hours: 0.0, image_prep: false, assistant_hours: 0.0, photographer: Photographer::Ken},
            expenses: 0,
            drone: false,
        }
    }

    pub fn calc_price(&self) -> f32{
        match &self.shoot_type{
            ShootType::Hourly { hours, image_prep, assistant_hours, photographer} => {
                self.calc_hourly(*hours, *image_prep, *assistant_hours, photographer)
            },
            ShootType::HalfDayBased { halves, image_prep, assistant_hours, photographer} => {
                self.calc_half_day(*halves, *image_prep, *assistant_hours, photographer)
            },
            ShootType::Headshot { hours, people, editing, retouch, assistant_hours, photographer } => {
                self.calc_headshot(*hours, *people, *editing, retouch, *assistant_hours, photographer)
            },
            ShootType::NewHeadshot {heads, headshot_type, editing} =>{
                self.calc_new_headshot(*heads, headshot_type, *editing)
            }
        }
    }

    fn calc_hourly(&self, hours: f32, image_prep: bool, assistant_hours: f32, photographer: &Photographer) -> f32{
        (photographer.get_hourly() * hours) 
        + if image_prep {IMAGE_PREP_PRICE as f32} else {0.0} + if self.drone {DRONE_PRICE as f32} else {0.0}
        + assistant_hours as f32 * ASSISTANT_PRICE as f32
        + self.expenses as f32 * EXPENSES_PRICE as f32
    }

    fn calc_half_day(&self, halves: u32, image_prep: bool, assistant_hours: f32, photographer: &Photographer) -> f32{
        ((halves as f32/2.0).ceil() * photographer.get_first_half_day())
        + ((halves as f32/2.0).floor() * photographer.get_second_half_day())
        + if image_prep {IMAGE_PREP_PRICE} else {0.0} + if self.drone {DRONE_PRICE} else {0.0}
        + assistant_hours * ASSISTANT_PRICE
        + self.expenses as f32 * EXPENSES_PRICE
    }

    fn calc_headshot(&self, hours: f32, people: u32, editing: bool, retouch_level: &RetouchLevel, assistant_hours: f32, photographer: &Photographer) -> f32{
        (photographer.get_hourly() * hours) 
        + if self.drone {DRONE_PRICE} else {0.0}
        + assistant_hours * ASSISTANT_PRICE
        + self.expenses as f32 * EXPENSES_PRICE
        + people as f32 * retouch_level.get_price_per()
        + if editing {ON_SITE_EDITIING_PRICE} else {0.0}
    }
    
    fn calc_new_headshot(&self, heads: u32, headshot_type: &HeadshotType, editing: bool) -> f32{
        let main_price = match headshot_type {
            HeadshotType::Large => {
                let hourly = calc_hours(heads) * (LARGE_HEADSHOT_HOURLY + (ASSISTANT_PRICE * 2.0));
                let per_person = heads as f32 * if heads > 10 {CHEAP_RETOUCH_PRICE} else {FANCY_RETOUCH_PRICE};

                hourly + per_person as f32 + if editing {ON_SITE_EDITIING_PRICE} else {0.0}
            },
            HeadshotType::Team => {
                let hourly = calc_hours(heads) * (TEAM_HEADSHOT_HOURLY + ASSISTANT_PRICE);
                let per_person = heads as f32 * CHEAP_RETOUCH_PRICE;

                hourly + per_person as f32 + if editing {ON_SITE_EDITIING_PRICE} else {0.0}
            }
            HeadshotType::Small => {
                300.0 //wow very fancy system
            }
        };
        main_price + self.expenses as f32 * EXPENSES_PRICE + if self.drone {DRONE_PRICE} else {0.0}
    }
}

pub fn calc_hours(heads: u32) -> f32{
    (heads as f32 / 12.0).ceil() + 1.0
}

pub enum ShootType{
    Hourly{
        hours: f32,
        image_prep: bool,
        assistant_hours: f32,
        photographer: Photographer,
    },
    HalfDayBased{
        halves: u32,
        image_prep: bool,
        assistant_hours: f32,
        photographer: Photographer,
    },
    Headshot{
        hours: f32,
        people: u32,
        editing: bool,
        retouch: RetouchLevel,
        assistant_hours: f32,
        photographer: Photographer,
    },
    NewHeadshot{
        heads: u32,
        headshot_type: HeadshotType,
        editing: bool,
    }
}
#[derive(PartialEq)]
pub enum HeadshotType{
    Large,
    Team,
    Small,
}

impl Display for HeadshotType{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            HeadshotType::Large => write!(f,"Large"),
            HeadshotType::Team => write!(f,"Team"),
            HeadshotType::Small => write!(f, "Small"),
        }
    }
}

impl Display for ShootType{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            ShootType::Hourly {..} => write!(f,"Hourly"),
            ShootType::HalfDayBased {..} => write!(f,"Half Day"),
            ShootType::Headshot {..} => write!(f,"Headshot"),
            ShootType::NewHeadshot {..} => write!(f, "Headshot"),
        }
    }
}

impl PartialEq for ShootType{
    fn eq(&self, other: &Self) -> bool {
        match (self, other){
            (ShootType::Hourly {..}, ShootType::Hourly {..}) |
            (ShootType::Headshot {..}, ShootType::Headshot {..}) |
            (ShootType::HalfDayBased {..}, ShootType::HalfDayBased {..}) |
            (ShootType::NewHeadshot {..}, ShootType::NewHeadshot {..}) => true,
            _ => false,
        }
    }
}

#[derive(PartialEq)]
pub enum Photographer{
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
pub enum RetouchLevel{
    Volume,
    Light,
    Nice,
}

impl RetouchLevel {
    pub fn get_price_per(&self) -> f32{
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
