extern crate cheddar;

fn main() {

    cheddar::Cheddar::new().expect("could not read manifest")
        .module("sum").expect("malformed module path")
        .run_build("include/sum.h");

    cheddar::Cheddar::new().expect("could not read manifest")
        .module("addition").expect("malformed module path")
        .run_build("include/addition.h");

    cheddar::Cheddar::new().expect("could not read manifest")
        .module("count_characters").expect("malformed module path")
        .run_build("include/count_characters.h");

    cheddar::Cheddar::new().expect("could not read manifest")
        .module("quotes").expect("malformed module path")
        .run_build("include/quotes.h");

    cheddar::Cheddar::new().expect("could not read manifest")
        .module("point").expect("malformed module path")
        .run_build("include/point.h");
}
