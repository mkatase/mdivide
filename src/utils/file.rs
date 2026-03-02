// src/utils/file.rs

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::ops::Range;
use std::path::{Path, PathBuf};
use crate::error::AppError;

// search round one, all tag indexing
pub fn capture_tags(input_path: PathBuf) -> Result<HashMap<String, Vec<Range<usize>>>, AppError> {
    let content = std::fs::read_to_string(&input_path).map_err(AppError::Io)?;
    
    let mut tag_map: HashMap<String, Vec<Range<usize>>> = HashMap::new();
    
    let mut stack: Vec<(usize, String)> = Vec::new();

    for (idx, line) in content.lines().enumerate() {
        if line.starts_with('@') {
            let tag_name = line[1..].trim_end();

            if tag_name == "end" {
                let (start_idx, label) = stack.pop().ok_or_else(|| {
                    AppError::InvalidTag(format!(
                        "Unmatched @end at line {} in {:?}",
                        idx + 1, input_path
                    ))
                })?;
                tag_map.entry(label).or_insert_with(Vec::new).push(start_idx..idx);
            } else {
                stack.push((idx, tag_name.to_string()));
            }
        }
    }

    if let Some((start_idx, label)) = stack.pop() {
        return Err(AppError::InvalidTag(format!(
            "Tag @{} (line {}) in {:?} is never closed",
            label, start_idx + 1, input_path
        )));
    }

    Ok(tag_map)
}

// Round two, sorting
pub fn write_extracted_content(
    output_path: &Path,
    input_path: &Path,
    tag_map: &HashMap<String, Vec<Range<usize>>>,
    target_lang: &str,
) -> Result<(), AppError> {
    // target tag + common tag
    let mut target_ranges = Vec::new();
    
    // target block
    if let Some(ranges) = tag_map.get(target_lang) {
        target_ranges.extend(ranges.clone());
    }
    
    // common block
    if let Some(ranges) = tag_map.get("common") {
        target_ranges.extend(ranges.clone());
    }

    target_ranges.sort_by_key(|r| r.start);

    let input_file = File::open(input_path)?;
    let reader = BufReader::new(input_file);

    let output_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output_path)
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::AlreadyExists {
                AppError::Config("Critical: File collision detected during write".into())
            } else {
                AppError::Io(e)
            }
        })?;

    let mut writer = BufWriter::new(output_file);

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    for range in target_ranges {
        for i in range.start + 1..range.end {
            if let Some(line) = lines.get(i) {
                writeln!(writer, "{}", line)?;
            }
        }
    }

    writer.flush()?;

    Ok(())
}
