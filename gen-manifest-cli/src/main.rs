use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

fn default_order() -> i32 {
    1000
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct DocMeta {
    title: String,
    #[serde(default)]
    slug: String,
    category: String,
    tags: Vec<String>,
    date: String,
    description: String,
    #[serde(default)]
    lang: String,
    #[serde(default = "default_order")]
    order: i32,
}

fn natural_cmp(a: &str, b: &str) -> std::cmp::Ordering {
    let mut chars_a = a.chars().peekable();
    let mut chars_b = b.chars().peekable();

    loop {
        match (chars_a.peek(), chars_b.peek()) {
            (None, None) => return std::cmp::Ordering::Equal,
            (None, Some(_)) => return std::cmp::Ordering::Less,
            (Some(_), None) => return std::cmp::Ordering::Greater,
            (Some(&ca), Some(&cb)) => {
                if ca.is_ascii_digit() && cb.is_ascii_digit() {
                    let mut num_a: u64 = 0;
                    while let Some(&c) = chars_a.peek() {
                        if c.is_ascii_digit() {
                            num_a = num_a * 10 + c.to_digit(10).unwrap() as u64;
                            chars_a.next();
                        } else {
                            break;
                        }
                    }
                    let mut num_b: u64 = 0;
                    while let Some(&c) = chars_b.peek() {
                        if c.is_ascii_digit() {
                            num_b = num_b * 10 + c.to_digit(10).unwrap() as u64;
                            chars_b.next();
                        } else {
                            break;
                        }
                    }
                    if num_a != num_b {
                        return num_a.cmp(&num_b);
                    }
                } else {
                    let ca_lower = ca.to_lowercase().to_string();
                    let cb_lower = cb.to_lowercase().to_string();
                    if ca_lower != cb_lower {
                        return ca_lower.cmp(&cb_lower);
                    }
                    chars_a.next();
                    chars_b.next();
                }
            }
        }
    }
}

fn main() {
    let mut manifest = Vec::new();
    let content_dir = Path::new("content");

    if !content_dir.exists() {
        eprintln!("Error: 'content' directory missing. Please ensure you are running this from the correct directory.");
        std::process::exit(1);
    }

    for entry in WalkDir::new(content_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "md") {
            let content = fs::read_to_string(path).expect("Failed to read file");
            let matter = gray_matter::Matter::<gray_matter::engine::YAML>::new();
            if let Some(data) = matter.parse(&content).data {
                let mut meta: DocMeta = data
                    .deserialize()
                    .expect("Metadata front-matter structural mismatch");
                let relative_path = path.strip_prefix(content_dir).unwrap();
                let components: Vec<String> = relative_path
                    .components()
                    .map(|c| c.as_os_str().to_string_lossy().into_owned())
                    .collect();

                meta.lang = components
                    .get(0)
                    .cloned()
                    .unwrap_or_else(|| "en".to_string());
                let slice_path = relative_path.with_extension("");
                let slug_path = slice_path.strip_prefix(&meta.lang).unwrap_or(&slice_path);
                meta.slug = slug_path.to_string_lossy().to_string();
                manifest.push(meta);
            }
        }
    }

    // Sort manifest items naturally to ensure sequential accuracy across languages, categories, and sections
    manifest.sort_by(|a, b| {
        if a.lang != b.lang {
            return a.lang.cmp(&b.lang);
        }
        if a.order != b.order {
            return a.order.cmp(&b.order);
        }
        if a.category != b.category {
            return natural_cmp(&a.category, &b.category);
        }
        natural_cmp(&a.slug, &b.slug)
    });

    let public_dir = Path::new("static");
    if !public_dir.exists() {
        fs::create_dir_all(public_dir).unwrap();
    }

    let dest_path = public_dir.join("meta.json");
    let json_data = serde_json::to_string_pretty(&manifest).unwrap();

    let should_write = match fs::read_to_string(&dest_path) {
        Ok(existing) => existing != json_data,
        Err(_) => true,
    };

    if should_write {
        fs::write(&dest_path, &json_data).expect("Failed writing meta.json");
        println!(
            "Manifest CLI Execution Success: Indexed {} records (Updated).",
            manifest.len()
        );
    }
}
