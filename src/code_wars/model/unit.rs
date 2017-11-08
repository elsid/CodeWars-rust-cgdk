pub trait Unit {
    fn id(&self) -> i64;
    fn x(&self) -> f64;
    fn y(&self) -> f64;
}

#[macro_export]
macro_rules! unit_impl(
    ($t:ty) => (
        impl Unit for $t {
            fn id(&self) -> i64 {
                self.id()
            }

            fn x(&self) -> f64 {
                self.x()
            }

            fn y(&self) -> f64 {
                self.y()
            }
        }
    )
);
