use simple_si_units::base::Time;

use crate::population::{Population, Subpopulation};

pub(super) fn rho0(population: Population, age: Time<f64>) -> f64 {
    match population {
        Population::ThinDisc(Subpopulation::Alive) => {
            if age < Time::from_Gyr(0.15) {
                4.0e-3
            } else if age < Time::from_Gyr(1.) {
                7.9e-3
            } else if age < Time::from_Gyr(2.) {
                6.2e-3
            } else if age < Time::from_Gyr(3.) {
                4.0e-3
            } else if age < Time::from_Gyr(5.) {
                4.0e-3
            } else if age < Time::from_Gyr(7.) {
                4.9e-3
            } else {
                6.6e-3
            }
        }
        Population::ThinDisc(Subpopulation::WhiteDwarf) => 3.96e-3,
        Population::ThickDisc(Subpopulation::Alive) => 1.34e-3,
        Population::ThickDisc(Subpopulation::WhiteDwarf) => 3.04e-4,
        Population::Spheroid => 9.32e-6,
        Population::Bulge => 0.,
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_diff, assert_ratio};

    use super::*;

    const ACC: f64 = 1.0e-3;

    #[test]
    fn rho0_is_correct() {
        let rho = rho0(
            Population::ThinDisc(Subpopulation::Alive),
            Time::from_Gyr(0.1),
        );
        assert_ratio!(4.0e-3, rho, ACC);

        let rho = rho0(
            Population::ThinDisc(Subpopulation::Alive),
            Time::from_Gyr(0.5),
        );
        assert_ratio!(7.9e-3, rho, ACC);

        let rho = rho0(
            Population::ThinDisc(Subpopulation::Alive),
            Time::from_Gyr(1.5),
        );
        assert_ratio!(6.2e-3, rho, ACC);

        let rho = rho0(
            Population::ThinDisc(Subpopulation::Alive),
            Time::from_Gyr(2.5),
        );
        assert_ratio!(4.0e-3, rho, ACC);

        let rho = rho0(
            Population::ThinDisc(Subpopulation::Alive),
            Time::from_Gyr(3.5),
        );
        assert_ratio!(4.0e-3, rho, ACC);

        let rho = rho0(
            Population::ThinDisc(Subpopulation::Alive),
            Time::from_Gyr(5.5),
        );
        assert_ratio!(4.9e-3, rho, ACC);

        let rho = rho0(
            Population::ThinDisc(Subpopulation::Alive),
            Time::from_Gyr(7.5),
        );
        assert_ratio!(6.6e-3, rho, ACC);

        let rho = rho0(
            Population::ThinDisc(Subpopulation::WhiteDwarf),
            Time::from_Gyr(0.),
        );
        assert_ratio!(3.96e-3, rho, ACC);

        let rho = rho0(
            Population::ThickDisc(Subpopulation::Alive),
            Time::from_Gyr(0.),
        );
        assert_ratio!(1.34e-3, rho, ACC);

        let rho = rho0(
            Population::ThickDisc(Subpopulation::WhiteDwarf),
            Time::from_Gyr(0.),
        );
        assert_ratio!(3.04e-4, rho, ACC);

        let rho = rho0(Population::Spheroid, Time::from_Gyr(0.));
        assert_ratio!(9.32e-6, rho, ACC);

        let rho = rho0(Population::Bulge, Time::from_Gyr(0.));
        assert_diff!(0., rho, ACC);
    }
}
