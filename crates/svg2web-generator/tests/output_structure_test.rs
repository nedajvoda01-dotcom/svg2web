use std::fs;

use svg2web_core::{analyze, parse};
use svg2web_generator::{GeneratorBuilder, OutputOptions};
use svg2web_generator::registry::RenderContext;

fn temp_dir(name: &str) -> std::path::PathBuf {
	let mut dir = std::env::temp_dir();
	dir.push(format!(
		"svg2web-output-structure-{}-{}-{}",
		name,
		std::process::id(),
		std::time::SystemTime::now()
			.duration_since(std::time::UNIX_EPOCH)
			.map(|d| d.as_nanos())
			.unwrap_or(0)
	));
	let _ = fs::create_dir_all(&dir);
	dir
}

#[test]
fn test_vanilla_structure_created() {
	let output = temp_dir("vanilla").join("dist");

	let svg = r#"<svg xmlns="http://www.w3.org/2000/svg"><rect width="10" height="10"/></svg>"#;
	let element = parse(svg).expect("parse should succeed");
	let analysis = analyze(&element);

	let generator = GeneratorBuilder::new()
		.format("vanilla")
		.build()
		.expect("generator should build");
	let context = RenderContext::new(element, analysis, vec![], "vanilla");
	let result = generator.render(&context).expect("render should succeed");

	use svg2web_generator::output::structure::OutputStructure;
	let structure = OutputStructure::new(&output);
	structure
		.write(&result, &OutputOptions::default())
		.expect("write should succeed");

	assert!(output.join("css/variables.css").exists());
	assert!(output.join("css/base.css").exists());
	assert!(output.join("assets/vectors").exists());
	assert!(output.join("js/components").exists());
	assert!(output.join("index.html").exists());

	let _ = fs::remove_dir_all(output.parent().unwrap_or(output.as_path()));
}

#[test]
fn test_react_structure_created() {
	let output = temp_dir("react").join("dist");

	let svg = r#"<svg xmlns="http://www.w3.org/2000/svg"><circle cx="5" cy="5" r="5"/></svg>"#;
	let element = parse(svg).expect("parse should succeed");
	let analysis = analyze(&element);

	let generator = GeneratorBuilder::new()
		.format("react")
		.build()
		.expect("generator should build");
	let context = RenderContext::new(element, analysis, vec![], "react");
	let result = generator.render(&context).expect("render should succeed");

	let structure = svg2web_generator::output::structure::OutputStructure::new(&output);
	structure
		.write(&result, &OutputOptions::default())
		.expect("write should succeed");

	assert!(output.join("src/components").exists());
	assert!(output.join("public").exists());
	assert!(output.join("package.json").exists());

	let _ = fs::remove_dir_all(output.parent().unwrap_or(output.as_path()));
}
