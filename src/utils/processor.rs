// src/utils/processor.rs

use std::collections::HashMap;
use std::ops::Range;
use std::path::Path;

use crate::{FileContext, FilePair};
use crate::utils::file;
use crate::error::AppError;

// for check mode
fn show_check_report(tag_map: &HashMap<String, Vec<Range<usize>>>, _content: &str, input_path: &Path) {
println!("\n[CHECK REPORT] Source: {}", input_path.display());
    println!("--------------------------------------------------");

    let mut tags: Vec<_> = tag_map.keys().collect();
    tags.sort_by(|a, b| {
        if *a == "common" { std::cmp::Ordering::Less }
        else if *b == "common" { std::cmp::Ordering::Greater }
        else { a.cmp(b) }
    });

    for tag in tags {
        if let Some(ranges) = tag_map.get(tag) {
            let mut total_effective_lines = 0;
            for r in ranges {
                if r.end > r.start + 1 {
                    total_effective_lines += r.end - (r.start + 1);
                }
            }
            println!("  @{:<8}: {:>3} blocks ({:>4} lines total)", tag, ranges.len(), total_effective_lines);
        }
    }
    println!("--------------------------------------------------\n");
}

// for filelist
fn extract_pure_basename(path_str: &str) -> String {
    let filename = path_str
        .split(['/', '\\'])
        .filter(|s| !s.is_empty())
        .last()
        .unwrap_or("unknown");

    match filename.rsplit_once('.') {
        Some((before_dot, _)) if !before_dot.is_empty() => before_dot.to_string(),
        _ => filename.to_string(),
    }
}

fn determine_output_path(pair: &FilePair, out_dir: &Option<String>) -> Result<std::path::PathBuf, AppError> {
    if !pair.output.is_empty() {
        Ok(std::path::PathBuf::from(&pair.output))
    } else {
        let pure_name = extract_pure_basename(&pair.input);
        let dir = out_dir.as_ref().ok_or_else(|| {
            AppError::Config("Output path undefined (-d is missing)".to_string())
        })?;
        Ok(std::path::Path::new(dir).join(format!("{}.md", pure_name)))
    }
}

// for skip mode
fn check_overwrite_protection(path: &std::path::Path, skip: bool) -> Result<bool, AppError> {
    if path.exists() {
        if skip {
            println!("  [SKIP] Already exists: {:?}", path);
            return Ok(true);
        } else {
            return Err(AppError::Config(
                format!("Markdown Asset Protection: Target file exists: {:?}", path)
            ));
        }
    }
    Ok(false)
}

fn execute_write(
    path: &std::path::Path,
    input: &std::path::Path,
    tags: &HashMap<String,Vec<Range<usize>>>,
    lang: &str,
) -> Result<(), AppError> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(AppError::Io)?;
    }
    file::write_extracted_content(path, input, tags, lang)?;
    println!("  [DONE] Generated: {:?}", path);
    Ok(())
}

// 
pub fn run_process(pair: &FilePair, ctx: &FileContext) -> Result<(), AppError> {
    println!("Processing: {}", pair.input);
    let input_path = Path::new(&pair.input);
    
    if !input_path.exists() {
        return Err(AppError::Config(
            format!("Input file not found: {}", pair.input)));
    }

    let tag_map = file::capture_tags(input_path.to_path_buf())?;

    if ctx.check {
        let input_content = std::fs::read_to_string(input_path).map_err(AppError::Io)?;
        show_check_report(&tag_map, &input_content, input_path);
        return Ok(());
    }

    let target_lang = ctx.lang.as_ref().unwrap();
    if !tag_map.contains_key(target_lang) {
        return Err(AppError::InvalidTag(format!(
            "Tag @{} not found in {}", target_lang, pair.input)));
    }

    let safe_output_path = determine_output_path(pair, &ctx.out_dir)?;

    println!("  -> Targeting: {:?}", safe_output_path);

    if check_overwrite_protection(&safe_output_path, ctx.skip)? {
        return Ok(());
    }

    execute_write(&safe_output_path, input_path, &tag_map, target_lang)
}
