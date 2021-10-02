use crate::{Amount, Energy, Pressure, Temperature, Volume};

scalar! {
    struct GasConstant(f64) {
        fn in_j_per_mol_k(joules_per_amount_kelvin) -> Self;
    }
}

impl GasConstant {
    pub const R: GasConstant = GasConstant::in_j_per_mol_k(8.314_462_618_153_24);
}

scalar_div!(GasConstant | Volume = PressurePerAmountTemperature);
scalar_div!(EnergyPerAmount | Temperature = GasConstant);
scalar_div!(EnergyPerTemperature | Amount = GasConstant);
scalar_div!(GasConstant | Pressure = VolumePerAmountTemperature);

scalar! {
    struct PressurePerAmountTemperature(f64) {
        fn in_pa_per_mol_k(pascal_per_amount_kelvin) -> Self;
    }
}

scalar! {
    struct EnergyPerAmount(f64) {
        fn in_j_per_mol(joules_per_amount) -> Self;
    }
}

scalar_div!(Energy | Amount = EnergyPerAmount);
scalar_div!(EnergyPerAmount | Volume = PressurePerAmount);
scalar_div!(EnergyPerAmount | Pressure = VolumePerAmount);

scalar! {
    struct EnergyPerTemperature(f64) {
        fn in_j_per_k(joules_per_kelvin) -> Self;
    }
}

scalar_div!(Energy | Temperature = EnergyPerTemperature);
scalar_div!(EnergyPerTemperature | Volume = PressurePerTemperature);
scalar_div!(EnergyPerTemperature | Pressure = VolumePerTemperature);

scalar! {
    struct VolumePerAmountTemperature(f64) {
        fn in_m3_per_mol_k(meters_cubed_per_amount_kelvin) -> Self;
    }
}

scalar! {
    struct PressurePerTemperature(f64) {
        fn in_pa_per_k(pascal_per_kelvin) -> Self;
    }
}

scalar_div!(PressurePerTemperature | Amount = PressurePerAmountTemperature);
scalar_div!(Pressure | Temperature = PressurePerTemperature);

scalar! {
    struct PressurePerAmount(f64) {
        fn in_pa_per_mol(pascal_per_amount) -> Self;
    }
}

scalar_div!(PressurePerAmount | Temperature = PressurePerAmountTemperature);
scalar_div!(Pressure | Amount = PressurePerAmount);

scalar! {
    struct VolumePerAmount(f64) {
        fn in_m3_per_mol(meters_cubed_per_amount) -> Self;
    }
}

scalar_div!(VolumePerAmount | Temperature = VolumePerAmountTemperature);
scalar_div!(Volume | Amount = VolumePerAmount);

scalar! {
    struct VolumePerTemperature(f64) {
        fn in_m3_per_k(meters_cubed_per_kelvin) -> Self;
    }
}

scalar_div!(VolumePerTemperature | Amount = VolumePerAmountTemperature);
scalar_div!(Volume | Temperature = VolumePerTemperature);

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn gas_constant_unit_composition() {
        const GC: GasConstant = GasConstant::in_j_per_mol_k(1.0);
        assert_eq!(GC, N / M2 * M3 / MOL / K);
        assert_eq!(MOL, PA * M3 / K / GC);
    }

    #[test]
    fn ideal_gas_state_change() {
        let p1 = PA;
        let t1 = K;
        let v1 = M3;

        let p2 = 2.0 * PA;
        let t2 = 0.5 * K;

        // p1 * v1 / t1 = p2 * v2 / t2
        let v2 = p1 * v1 / t1 / p2 * t2;

        assert_eq!(0.25 * v1, v2);
    }
}
