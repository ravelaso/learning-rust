use crate::classes::{
    class::{calculator_test, person_test},
    complex_pattern::complex_test,
    patterns::configuration_test,
};

pub mod class;
pub mod complex_pattern;
pub mod patterns;

pub fn test() {
    person_test();
    calculator_test();
    configuration_test();
    complex_test();
}
