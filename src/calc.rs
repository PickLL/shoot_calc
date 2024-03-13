use core::fmt::Display;

const IMAGE_PREP_PRICE: f32 = 50.0;
const HIGHER_IMAGE_PREP_PRICE: f32 = 50.0 * 2.0;

const DRONE_PRICE: f32 = 150.0;

const ASSISTANT_PRICE: f32 = 40.0;
const HIGHER_ASSISTANT_PRICE: f32 = 50.0;

const EXPENSES_PRICE: f32 = 10.0;
const ON_SITE_EDITIING_PRICE: f32 = 100.0;

const LARGE_HEADSHOT_HOURLY: f32 = 275.0;
const TEAM_HEADSHOT_HOURLY: f32 = 200.0;

const CHEAP_RETOUCH_PRICE: f32 = 10.0;
const FANCY_RETOUCH_PRICE: f32 = 20.0;

const CONFERENCE_HOURLY: f32 = 200.0;

pub struct CalcApp {
    pub shoot_type: ShootType,
    pub expenses: u32,
    pub drone: bool,
}

impl CalcApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> CalcApp {
        CalcApp {
            shoot_type: ShootType::Hourly {
                hours: 0.0,
                image_prep: false,
                assistant_hours: 0.0,
                use_higher_assistant_price: false,
                use_higher_prep_price: false,
                photographer: Photographer::Ken,
            },
            expenses: 0,
            drone: false,
        }
    }

    pub fn calc_price(&self) -> f32 {
        match &self.shoot_type {
            ShootType::Hourly {
                hours,
                image_prep,
                assistant_hours,
                photographer,
                use_higher_prep_price,
                use_higher_assistant_price,
            } => self.calc_hourly(*hours, *image_prep, *assistant_hours, photographer, *use_higher_prep_price, *use_higher_assistant_price),
            ShootType::HalfDayBased {
                halves,
                image_prep,
                assistant_hours,
                photographer,
                use_higher_prep_price,
                use_higher_assistant_price,
            } => self.calc_half_day(*halves, *image_prep, *assistant_hours, photographer, *use_higher_prep_price, *use_higher_assistant_price),
            ShootType::Headshot {
                heads,
                headshot_type,
                editing,
                retouch_level,
                extra_retouched_photos,
                days,
            } => self.calc_headshot(*heads, headshot_type, *editing, retouch_level, *extra_retouched_photos, *days),
            ShootType::Conference { hours, extra_cost } => self.calc_conference(*hours, *extra_cost),
        }
    }

    fn calc_hourly(
        &self,
        hours: f32,
        image_prep: bool,
        assistant_hours: f32,
        photographer: &Photographer,
        use_higher_prep_price: bool,
        use_higher_assistant_price: bool,        
    ) -> f32 {
        (photographer.get_hourly() * hours)
            + if image_prep {
                if use_higher_prep_price {HIGHER_IMAGE_PREP_PRICE} else {IMAGE_PREP_PRICE}
            } else {
                0.0
            }
            + if self.drone { DRONE_PRICE} else { 0.0 }
            + assistant_hours * if use_higher_assistant_price {HIGHER_ASSISTANT_PRICE} else {ASSISTANT_PRICE}
            + self.expenses as f32 * EXPENSES_PRICE
    }

    fn calc_half_day(
        &self,
        halves: u32,
        image_prep: bool,
        assistant_hours: f32,
        photographer: &Photographer,
        use_higher_prep_price: bool,
        use_higher_assistant_price: bool,
    ) -> f32 {
        ((halves as f32 / 2.0).ceil() * photographer.get_first_half_day())
            + ((halves as f32 / 2.0).floor() * photographer.get_second_half_day())
            + if image_prep {
                if use_higher_prep_price {HIGHER_IMAGE_PREP_PRICE} else {IMAGE_PREP_PRICE}
            } else {
                0.0
            }
            + if self.drone { DRONE_PRICE } else { 0.0 }
            + assistant_hours * if use_higher_assistant_price {HIGHER_ASSISTANT_PRICE} else {ASSISTANT_PRICE}
            + self.expenses as f32 * EXPENSES_PRICE
    }

    fn calc_headshot(&self, heads: u32, headshot_type: &HeadshotType, editing: bool, retouch_level: &RetouchLevel, extra_retouched_photos: u32, days: u32) -> f32 {
        let main_price = match headshot_type {
            HeadshotType::Large => {
                let hourly = calc_hours(heads) * (LARGE_HEADSHOT_HOURLY + (ASSISTANT_PRICE * 2.0));
                let per_person = heads as f32 * retouch_level.get_price_per();
                let extra_retouch = extra_retouched_photos as f32 * 20.0;
                hourly + per_person + extra_retouch + if editing { ON_SITE_EDITIING_PRICE * days as f32 } else { 0.0 }
            }
            HeadshotType::Team => {
                let hourly = calc_hours(heads) * (TEAM_HEADSHOT_HOURLY + ASSISTANT_PRICE);
                let per_person = heads as f32 * retouch_level.get_price_per();
                let extra_retouch = extra_retouched_photos as f32 * 20.0;

                hourly + per_person + extra_retouch + if editing { ON_SITE_EDITIING_PRICE * days as f32 } else { 0.0 }
            }
            HeadshotType::Small => {
                400.0 //wow very fancy system
            }
        };
        main_price
            + self.expenses as f32 * EXPENSES_PRICE
            + if self.drone { DRONE_PRICE } else { 0.0 }
    }

    fn calc_conference(&self, hours: f32, extra_cost: f32) -> f32{
        hours * CONFERENCE_HOURLY
        + if self.drone { DRONE_PRICE } else { 0.0 }
        + self.expenses as f32 * EXPENSES_PRICE
        + extra_cost
    }
}

pub fn calc_hours(heads: u32) -> f32 {
    (heads as f32 / 12.0).ceil() + 1.0
}

pub enum ShootType {
    Hourly {
        hours: f32,
        image_prep: bool,
        assistant_hours: f32,
        use_higher_prep_price: bool,
        use_higher_assistant_price: bool,
        photographer: Photographer,
    },
    HalfDayBased {
        halves: u32,
        image_prep: bool,
        assistant_hours: f32,
        use_higher_prep_price: bool,
        use_higher_assistant_price: bool,
        photographer: Photographer,
    },
    Headshot {
        heads: u32,
        headshot_type: HeadshotType,
        retouch_level: RetouchLevel,
        editing: bool,
        extra_retouched_photos: u32,
        days: u32,
    },
    Conference{
        hours: f32,
        extra_cost: f32,
    }
}
#[derive(PartialEq)]
pub enum HeadshotType {
    Large,
    Team,
    Small,
}

impl Display for HeadshotType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HeadshotType::Large => write!(f, "Large"),
            HeadshotType::Team => write!(f, "Team"),
            HeadshotType::Small => write!(f, "Small"),
        }
    }
}

impl Display for ShootType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShootType::Hourly { .. } => write!(f, "Hourly"),
            ShootType::HalfDayBased { .. } => write!(f, "Half Day"),
            ShootType::Headshot { .. } => write!(f, "Headshot"),
            ShootType::Conference { .. } => write!(f, "Conference"),
        }
    }
}

impl PartialEq for ShootType {
    fn eq(&self, other: &Self) -> bool {
        matches!((self, other),
            (ShootType::Hourly { .. }, ShootType::Hourly { .. })
            | (ShootType::HalfDayBased { .. }, ShootType::HalfDayBased { .. })
            | (ShootType::Headshot { .. }, ShootType::Headshot { .. })
        )
    }
}

#[derive(PartialEq)]
pub enum Photographer {
    Ken,
    Colin,
    Team,
}

impl Display for Photographer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Photographer::Ken => write!(f, "Ken"),
            Photographer::Colin => write!(f, "Colin"),
            Photographer::Team => write!(f, "Team"),
        }
    }
}

impl Photographer {
    fn get_hourly(&self) -> f32 {
        match self {
            Photographer::Ken => 275.0,
            Photographer::Colin => 225.0,
            Photographer::Team => 150.0,
        }
    }
    fn get_first_half_day(&self) -> f32 {
        match self {
            Photographer::Ken => 1500.0,
            Photographer::Colin => 1500.0,
            Photographer::Team => 600.0,
        }
    }
    fn get_second_half_day(&self) -> f32 {
        match self {
            Photographer::Ken => 1000.0,
            Photographer::Colin => 1000.0,
            Photographer::Team => 600.0,
        }
    }
}

#[derive(PartialEq)]
pub enum RetouchLevel {
    Student,
    Discount,
    Corporate,
    Full,
}

impl RetouchLevel {
    pub fn get_price_per(&self) -> f32 {
        match self {
            RetouchLevel::Student => 5.0,
            RetouchLevel::Discount => 10.0,
            RetouchLevel::Corporate => 20.0,
            RetouchLevel::Full => 50.0,
        }
    }
}

impl Display for RetouchLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RetouchLevel::Student => write!(f, "Student ${}", self.get_price_per()),
            RetouchLevel::Discount => write!(f, "Discount ${}", self.get_price_per()),
            RetouchLevel::Corporate => write!(f, "Corporate/Under 20 People ${}", self.get_price_per()),
            RetouchLevel::Full => write!(f, "Full Price/Special Needs ${}", self.get_price_per()),
        }
    }
}
