mod steps;
mod debug;
mod utils;

use std::path::PathBuf;
use clap::{error::{Error, Result}, CommandFactory, Parser};
use crate::steps::{make_chunk_matches, render};
use crate::utils::{get_number, get_wav_reader, normalize_if_required, verify_file_extension, IntoMutSamplesVec};

// TODO: in order of importance:
// 2. dynamic chunking stuff
// 3. the other non i16 formats
// 4. option for gradient descent for search jump
// overwriting prevention

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
	/// The audio that you want to transform your input into
	#[arg[short = 't']]
	target: PathBuf,

	/// The audio that you are using to create your target
	#[arg[short = 'p']]
	paint: PathBuf,

	/// The path of the output
	#[arg[short = 'o', default_value = "./out.wav"]]
	output: PathBuf,

	// /// Split the target into chunks of variable length, using shorter pieces for more interesting parts
	// variable_chunking: Option<bool>,

	/// When splitting the target clip, what size should the chunks be?
	#[arg[short = 'c', default_value = "500"]]
	chunk_size: usize,

	/// When searching through the paint input to find a match, how many samples should we advance the search head on each iteration?
	/// The lower this number, the more accurate the results will be
	#[arg[short = 'j', default_value = "200"]]
	search_jump: usize,

	/// Dry/wet mix between original target and resulting output. Think of this as "percentage dry"
	#[arg[short = 'm', default_value = "0"]]
	dry_wet_mix: f32,

	/// Normalize the volume of the target and paint audio
	#[arg[short = 'n']]
	normalize: bool,
}

fn execute() -> Result<(), Error> {
	let cli = Cli::parse();
	let mut command = Cli::command();

	let target_path = verify_file_extension(cli.target, &mut command)?;
	let paint_path = verify_file_extension(cli.paint, &mut command)?;

	let chunk_size = get_number(cli.chunk_size, 200, 10000, "chunk_size", &mut command)?;
	let dry_wet_mix = get_number(cli.dry_wet_mix, 0., 1., "dry_wet_mix", &mut command)?;
	let search_jump = get_number(cli.search_jump, 1, 1000, "search_jump", &mut command)?;

	let mut target_reader = get_wav_reader(target_path, &mut command)?;
	let mut paint_reader = get_wav_reader(paint_path, &mut command)?;

	let target_samples = normalize_if_required(target_reader.collect_samples(), cli.normalize);
	let paint_samples = normalize_if_required(paint_reader.collect_samples(), cli.normalize);

	let chunk_matches = make_chunk_matches(&target_samples, &paint_samples, chunk_size, search_jump)?;

	render(&chunk_matches, &target_samples, &paint_samples, chunk_size, cli.output, dry_wet_mix)?;

	Ok(())
}

fn main() {
	if let Err(err) = execute() {
		err.exit();
	}
}