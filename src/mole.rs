use crate::Mass;

pub const MOL: Mole = Mole::in_mol(1.0);

scalar! {
    struct Mole(f64) {
        fn in_mol(moles) -> Self;
    }
}

scalar! {
    struct MolecularMass(f64) {
        fn in_kg_per_mol(kilograms_per_mole) -> Self;
    }
}

impl MolecularMass {
    pub fn in_g_per_mol(grams_per_mole: f64) -> Self {
        Self::in_kg_per_mol(grams_per_mole * 1e-3)
    }
}

scalar_div!(Mass | Mole = MolecularMass);
