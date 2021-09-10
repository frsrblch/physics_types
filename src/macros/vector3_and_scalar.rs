#[macro_export]
macro_rules! vector3_and_scalar {
    {
        struct $vector:ident([struct $scalar:ident($base:ty);2]) {
            fn $abrev:ident($unit:ident) -> Self;
        }
    } => {
        vector_and_scalar! {
            struct $vector([struct $scalar($base);2]) {
                fn $abrev($unit) -> Self;
            }
        }

        paste::paste! {
            vector3! {
                struct [<$vector 3>] ([$scalar; 3]) {
                    fn $abrev($unit: $base) -> Self;
                }
            }

            impl const std::ops::Mul<$scalar> for ($base, $base, $base) {
                type Output = [<$vector 3>];

                fn mul(self, rhs: $scalar) -> Self::Output {
                    [<$vector 3>] {
                        x: self.0 * rhs,
                        y: self.1 * rhs,
                        z: self.2 * rhs,
                    }
                }
            }

            impl const std::ops::Mul<&$scalar> for ($base, $base, $base) {
                type Output = [<$vector 3>];

                fn mul(self, rhs: &$scalar) -> Self::Output {
                    self * *rhs
                }
            }
        }


    };
}
