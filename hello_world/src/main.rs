fn main() {
    println!("Hello, world!");
    println!("{0}, {1}, {2}, {3}, {4}", simnple_lib::simple_math::add(10, 20), simnple_lib::simple_math::add(1, 2), simnple_lib::simple_math::add(3, 7), simnple_lib::add_macro!(10, 10), simnple_lib::add_macro!(20, 20));
}
