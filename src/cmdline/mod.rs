pub mod parser;

#[derive(Default)]
pub struct Options {
	// Files
	pub png_file_name: String,
	pub out_file_name: String,

	//

	// Behaviour
	pub verbose: bool,
}