//@  compile-flags: --edition=2021

// This test ensures that expr_2021 is not allowed on pre-2024 editions

macro_rules! m {
    ($e:expr_2021) => { //~ ERROR: invalid fragment specifier `expr_2021`
        $e
    };
}

fn main() {
    m!(()); //~ ERROR: no rules expected the token `(`
}
