#![allow(dead_code, unused_variables)]
use crate::checks::BackwardTree;
//use crate::render::constants;
use crate::types::{State};

use svg::Document;
// use svg::node::element::Text;
// use svg::node::element::Path;
// use svg::node::element::path::Data;

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
	let document = Document::new()
	    .set("viewBox", (0,0,1000,1000));

	// let mut vertOffset = 0;
	// for i in &self.tree.ingredients.iter() {
	//     vertOffset = vertOffset + 20;
	//     let name = Text::new(&self.state[i.stuff]);
	//     let amount = Text::new(i.amount.map(|amt| &self.state[*amt]));
	//     amount.set("x", 0);
	//     amount.set("y", vertOffset);

	//     name.set("x", 20);
	//     name.set("y", vertOffset);
	    
	//     document.add(amount);
	//     document.add(name);
	// }

	
	// let data = Data::new()
	//     .move_to((10,10))
	//     .line_by((0,100))
	//     .line_by((50,0))
	//     .line_by((0, -50))
	//     .close();

	// let path = Path::new()
	//     .set("fill", "none")
	//     .set("stroke", "black")
	//     .set("stroke-width", 3)
	//     .set("d", data);

	// let text = Text::new("A recipe!")
	//     .set("x", 50)
	//     .set("y", 60);
//	    .set("rotate", 90); // rotates each *letter*
	
	svg::save("image.svg", &document).unwrap();

	format!("Wrote file")
    }
}
