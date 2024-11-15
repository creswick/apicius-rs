#![allow(dead_code, unused_variables)]
use crate::checks::{BackwardTree};
use crate::types::{ActionStep, IngredientRef, State};

pub struct RenderConfig {}

pub struct Graph<'a> {
    state: &'a State,
}

impl<'a> Graph<'a> {
    pub fn new(state: &'a State) -> Graph<'a> {
        Graph { state }
    }

    pub fn draw(&self, tree: &'a BackwardTree) {
        // there's a weird thing that's going to happen here, because
        // we don't yet know how big to make the surface. That means
        // that we're going to draw onto a big image surface but might
        // need to redraw if we run out of space
//        let surface = cairo::ImageSurface::create(cairo::Format::Rgb24, 2048, 2048)?;
//        let ctx = cairo::Context::new(&surface)?;

	println!("Drawing:");
	println!("  Ingredients: {}", tree.ingredients.len());
	println!("  Actions: {}", tree.actions.len());
	println!("  Paths: {}", tree.paths.len());

	let actionCount = tree.actions.len();
	
        for iref in tree.ingredients.iter() {
	    println!("ingredient");
	    let i = &self.state[*iref];
	    let name = &self.state[i.stuff];
	    let optAmount = i.amount.map(|amt| &self.state[*amt]);
	    println!("name {} amount {}", name, optAmount.unwrap_or("<none>"));
        }


    }
}
