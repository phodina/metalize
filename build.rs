extern crate cheddar;

fn main() {

	println!("Generating header for sum ...");
    cheddar::Cheddar::new().expect("could not read manifest")
        .module("sum").expect("malformed module path")
        .run_build("include/sum.h");

	println!("Generating header for additon ...");
    cheddar::Cheddar::new().expect("could not read manifest")
        .module("addition").expect("malformed module path")
        .run_build("include/addition.h");

	println!("Generating header for count_characters ...");
    cheddar::Cheddar::new().expect("could not read manifest")
        .module("count_characters").expect("malformed module path")
        .run_build("include/count_characters.h");

	println!("Generating header for quotes ...");
    cheddar::Cheddar::new().expect("could not read manifest")
        .module("quotes").expect("malformed module path")
        .run_build("include/quotes.h");

	println!("Generating header for point ...");
    cheddar::Cheddar::new().expect("could not read manifest")
        .module("point").expect("malformed module path")
        .run_build("include/point.h");

	println!("Generating header for account ...");
    cheddar::Cheddar::new().expect("could not read manifest")
        .module("accounts").expect("malformed module path")
        .run_build("include/accounts.h");
}
