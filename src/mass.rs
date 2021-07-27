use super::*;

pub const KG: Mass = Mass::in_kg(1.0);
pub const TON: Mass = Mass::in_kg(1e3);

scalar! {
    struct Mass(f64) {
        fn in_kg(kilograms) -> Self;
    }
}

#[test]
fn request_enough() {
    let mut mass = Mass::in_kg(3.0);
    let amount = Mass::in_kg(2.0);

    let actual = mass.request(amount);
    let expected = Mass::in_kg(2.0);

    assert_eq!(mass, Mass::in_kg(1.0));
    assert_eq!(actual, expected);
}

#[test]
fn request_insufficient() {
    let mut mass = Mass::in_kg(2.0);
    let amount = Mass::in_kg(3.0);

    let actual = mass.request(amount);
    let expected = Mass::in_kg(2.0);

    assert_eq!(mass, Mass::zero());
    assert_eq!(actual, expected);
}

impl Display for Mass {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:.0} kg", self.value)
    }
}

#[test]
fn mass_display() {
    assert_eq!("25 kg", Mass::in_kg(25.0).to_string());
}

impl Mass {
    pub fn tons(self) -> Tons {
        Tons(self)
    }
}

pub struct Tons(Mass);

impl Display for Tons {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let tons = (self.0.value / 1e3) as i64;
        write!(f, "{} t", tons.to_formatted_string(&Locale::en))
    }
}
