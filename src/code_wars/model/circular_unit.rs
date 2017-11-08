use super::unit::Unit;

pub trait CircularUnit: Unit {
    fn radius(&self) -> f64;
}

#[macro_export]
macro_rules! circular_unit_impl(
    ($t:ty) => (
        impl CircularUnit for $t {
            fn radius(&self) -> f64 {
                self.radius()
            }
        }
    )
);
