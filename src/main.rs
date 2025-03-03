use std::{
    collections::HashMap,
    env, fs,
    io::{self, Write},
    time::Instant,
};

fn generate_html(output_path: &str, values: HashMap<String, String>) -> io::Result<()> {
    let template = fs::read_to_string("template.html")?;
    let output = values.iter().fold(template, |acc, (key, value)| {
        acc.replace(&format!("{{{{{}}}}}", key), value)
    });
    fs::write(output_path, output.as_bytes())?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <output_path> KEY=VALUE ...", args[0]);
        return;
    }

    let output_path = &args[1];
    let values = args[2..]
        .iter()
        .filter_map(|arg| {
            arg.split_once('=').and_then(|(key, value)| {
                if !value.is_empty() {
                    Some((key.to_string(), value.to_string()))
                } else {
                    None
                }
            })
        })
        .collect::<HashMap<_, _>>();

    let start = Instant::now();
    if let Err(e) = generate_html(output_path, values) {
        eprintln!("Failed to generate HTML: {:?}", e);
    }
    println!("HTML generation took: {} ms", start.elapsed().as_millis());
}
