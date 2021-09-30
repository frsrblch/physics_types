use super::*;

scalar! {
    struct MassRate(f64) {
        fn in_kg_per_s(kg_per_second) -> Self;
    }
}

impl MassRate {
    #[inline]
    pub const fn in_tons_per_day(tons_per_day: f64) -> Self {
        tons_per_day * TONNE / DAY
    }

    #[inline]
    pub fn tons_per_day(self) -> TonsPerDay {
        TonsPerDay(self)
    }
}

pub struct TonsPerDay(MassRate);

impl Display for TonsPerDay {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let tons_per_day = (self.0.value / 1e3 * Duration::SECONDS_PER_DAY) as i64;
        write!(f, "{} t/day", tons_per_day.to_formatted_string(&Locale::en))
    }
}

scalar_div! { Mass | Duration = MassRate }
scalar_div! { MassRate | Frequency = Mass }

scalar! {
    struct MassRatePerPerson(f64) {
        fn in_kg_per_s_person(kg_per_person_second) -> Self;
    }
}

pub type Productivity = MassRatePerPerson;

scalar_div! { MassRate | Population = MassRatePerPerson }
