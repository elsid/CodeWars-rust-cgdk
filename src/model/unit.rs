pub trait Unit {
    fn id(&self) -> i64;
    fn x(&self) -> f64;
    fn y(&self) -> f64;

    fn distance_to(&self, x: f64, y: f64) -> f64 {
        (self.x() - x).hypot(self.y() - y)
    }

    fn distance_to_unit<T: Unit>(&self, unit: &T) -> f64 {
        self.distance_to(unit.x(), unit.y())
    }

    fn squared_distance_to(&self, x: f64, y: f64) -> f64 {
        let dx = x - self.x();
        let dy = y - self.y();
        dx * dx + dy * dy
    }

    fn squared_distance_to_unit<T: Unit>(&self, unit: &T) -> f64 {
        self.squared_distance_to(unit.x(), unit.y())
    }
}

#[macro_export]
macro_rules! unit_impl(
    ($t:ty) => (
        impl Unit for $t {
            fn id(&self) -> i64 {
                self.id
            }

            fn x(&self) -> f64 {
                self.x
            }

            fn y(&self) -> f64 {
                self.y
            }
        }
    )
);
