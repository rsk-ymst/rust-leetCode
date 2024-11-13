
struct Solution;

// Kelvin = Celsius + 273.15
// Fahrenheit = Celsius * 1.80 + 32.00

impl Solution {
    pub fn convert_temperature(celsius: f64) -> Vec<f64> {
        let kelvin = celsius + 273.15;
        let fahrenheit = celsius * 1.80 + 32.00;

        vec![kelvin, fahrenheit]
    }
}

pub fn main() {

}