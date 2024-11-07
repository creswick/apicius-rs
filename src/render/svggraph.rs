use crate::checks::BackwardTree;
use crate::render::constants;
use crate::types::{ActionStep, IngredientRef, State};

pub struct Graph<'a> {
    tree: &'a BackwardTree,
    state: &'a State,
}

impl<'a> Graph<'a> {

    pub fn new(s: &'a State, bt: &'a BackwardTree) -> Graph<'a> {
	Graph {
	    tree: bt,
	    state: s,
	}
    }
    
    pub fn render(&self) -> String {
	format!("producing graph")
    }
}
