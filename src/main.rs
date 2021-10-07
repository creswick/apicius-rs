#[macro_use] extern crate lalrpop_util;

pub mod types;

lalrpop_mod!(pub grammar);

const SAMPLE: &'static str = "
nicer scrambled eggs {
  [1/2] onion + [1 clove] garlic
    -> chop coarsely -> sautee & butter -> $mix;
  [2] eggs -> whisk -> $mix;
  $mix -> stir & salt -> DONE;
}
";

fn main() {
    let mut s = types::State::new();
    let recipe = grammar::RecipeParser::new().parse(&mut s, SAMPLE);
    println!("{:?}", recipe);
    assert!(recipe.is_ok());
    s.debug_recipe(recipe.unwrap());
}
