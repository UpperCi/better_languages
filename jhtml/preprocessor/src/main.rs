use std::{fs, env};
use std::path::PathBuf;
use serde_json::{Result, Value};

#[derive(Debug)]
enum Attribute {
	Tag(String, Vec<Attribute>),
	Text(String),
	Attribute(String, Option<String>)
}

fn parse_value(key: String, json_value: Value) -> Attribute {
	match json_value {
		Value::String(s) => {
			if key == "content" {
				Attribute::Text(s)
			} else {
				Attribute::Attribute(key, Some(s))
			}
		},
		Value::Object(obj) => {
			Attribute::Tag(
				key,
				obj.into_iter()
					.map(|(k, v)| parse_value(k, v))
					.collect()
			)
		}
		_ => { Attribute::Text(String::from("hi"))}
	}
}

fn assemble_html(attr: Attribute) -> String {
	match attr {
		Attribute::Text(s) => s,
		Attribute::Attribute(key, val) => {
			if let Some(s) = val {
				format!("{}=\"{}\"", key, s)
			} else {
				format!("{}=", key)
			}
		},
		Attribute::Tag(key, content) => {
			let (attrs, children): (Vec<Attribute>, Vec<Attribute>) = content.into_iter()
				.partition(|a| if let Attribute::Attribute(_, _) = a {true} else {false});

			let children_string: String = children.into_iter()
				.map(assemble_html)
				.collect::<Vec<String>>()
				.join(" ");
			let attrs_string: String = attrs.into_iter()
				.map(assemble_html)
				.collect::<Vec<String>>()
				.join(" ");
			format!("<{} {}>{}</{}>", key, attrs_string, children_string, key)
		}
	}
}

fn handle_path(path: PathBuf) {
	if fs::metadata(&path).unwrap().is_dir() {
		for p in fs::read_dir(&path).unwrap() {
			handle_path(p.unwrap().path());
		}
	} else {
		if let Some(ex) = path.extension() {
			if ex == "jhtml" {
				println!("{:?}", &path);
				let contents = fs::read_to_string(&path).unwrap();
				let root: Attribute = parse_value(String::from("html"),
					serde_json::from_str(
					contents.as_str()).unwrap());
				let mut new_path = path.clone();
				new_path.set_extension("html");
				fs::write(new_path, format!("{}\n", assemble_html(root)));
			}
		}
	}
}

fn main() -> Result<()> {
	let args: Vec<String> = env::args().collect();
	let paths: Vec<PathBuf> = args.into_iter().skip(1).map(|s| PathBuf::from(s)).collect();
	for p in paths {
		handle_path(p);
	}
	Ok(())
}
