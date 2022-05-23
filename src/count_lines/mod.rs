use std::fs::File;
use std::io::{BufRead, BufReader};
use walkdir::WalkDir;

fn count_lines(path: &str, ext: &str) -> u64 {
    let mut cnt = 0;
    for entry in WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        let f_path = entry.path().to_string_lossy();
        if f_name.ends_with(ext) {
            let file = BufReader::new(File::open(f_path.to_string()).expect("Unable to open file"));
            for _ in file.lines() {
                cnt += 1;
            }
        }
    }
    println!("total number of lines: {}", cnt);
    cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_walk_through() {
        assert_eq!(count_lines(".", ".rs"), 210);
    }
}
