pub const K: Temperature = Temperature::in_k(1.0);

scalar! {
    struct Temperature(f64) {
        fn in_k(kelvin) -> Self;
    }
}

impl Temperature {
    pub const fn in_c(celsius: f64) -> Self {
        Self::in_k(celsius + 273.15)
    }
}
