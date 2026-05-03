pub mod simple_math {
    pub fn add(left: u64, right: u64) -> u64 {
        left + right
    }

    #[macro_export]
    macro_rules!  add_macro{
        ($left : expr, $right : expr) => {
            $left + $right
        };
    }
}
