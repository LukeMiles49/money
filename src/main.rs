mod helpers;
mod style;

use list_files_macro::list_files;
use sankey::{Sankey, SankeyStyle};

use std::path::Path;

macro_rules! run_file {
	($x:expr) => {
		{
			#[path = $x]
			mod file;
			
			let name = Path::new($x).file_stem().unwrap().to_str().unwrap();
			println!("Generating out/{name}.svg");
			
			let mut sankey = Sankey::new();
			file::generate(&mut sankey);
			let style = SankeyStyle {
				number_format: Some(|x| format!("£{x:.2}")),
				..SankeyStyle::default()
			};
			let svg = sankey.draw(1280.0, 800.0, style);
			svg::save(format!("./out/{name}.svg"), &svg).unwrap();
		}
	};
}

fn main() {
	list_files!(run_file, "../data/*.rs");
}
