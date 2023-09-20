use std::io;

pub fn read_n_lines<'a>(n: u32, lines: &'a mut impl Iterator<Item = io::Result<String>>) -> io::Result<String> {
    let mut res = String::new();
    for _i in 0..n {
        let line = lines.next();
        match line {
            Some(Ok(l)) => {
                res.push_str(&l);
                res.push('\n');
            }
            Some(Err(e)) => return Err(e),
            None => break,
        }
    }
    Ok(res)
}

pub fn read_until_line_starts_with<'a>(matchstr: &'a str, lines: &'a mut impl Iterator<Item = io::Result<String>>) -> io::Result<String> {
    let mut res = String::new();
    loop {
        let Some(line) = lines.next() else { break };
        match line {
            Ok(l) => if l.starts_with(matchstr) { 
                break 
            } else { 
                res.push_str(&l);
                res.push('\n');
            },
            Err(e) => return Err(e),
        }
    }
    Ok(res)
}

pub fn filter_out_comment_lines(comment_char: char, lines: impl Iterator<Item = io::Result<String>>) -> impl Iterator<Item = io::Result<String>> {
    lines.filter(
        move |l| l.as_ref().is_ok_and(|line| !line.starts_with(comment_char)) 
                 | l.is_err()
    )
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::io::BufReader;
    use std::io::Cursor;
    use std::fs::File;
    use std::io::BufRead;
    #[test]
    fn test_read_n_lines() {
        let mut file = BufReader::new(File::open("tests/read_n_lines_test").unwrap()).lines(); 
        let first_three_lines = read_n_lines(3, &mut file).unwrap();
        assert_eq!(first_three_lines, "1\n2\n3\n");
        let Some(Ok(fourth_line)) = file.next() else {panic!("reading fourth line failed")};
        assert_eq!(fourth_line, "4");
    }

    #[test]
    fn test_read_n_lines_from_string() {
        let mut string_iter = Cursor::new("1\n2\n3\n4\n".to_string()).lines();
        let first_three_lines = read_n_lines(3, &mut string_iter).unwrap();
        assert_eq!(first_three_lines, "1\n2\n3\n");
        let Some(Ok(fourth_line)) = string_iter.next() else {panic!("reading fourth line failed")};
        assert_eq!(fourth_line, "4");
    }

    #[test]
    fn test_read_too_many_lines() {
        let mut string_iter = Cursor::new("1\n2\n3\n4\n").lines();
        let first_six_lines = read_n_lines(6, &mut string_iter).unwrap();
        assert_eq!(first_six_lines, "1\n2\n3\n4\n");
    }

    #[test]
    fn test_filter_out_lines_from_input() {
        let string_iter = Cursor::new("1\n#2\n3\n4\n#5\n6\n").lines();
        let mut filtered_iter = filter_out_comment_lines('#', string_iter);
        let first_three_lines_uncommented = read_n_lines(3, &mut filtered_iter).unwrap();
        assert_eq!(first_three_lines_uncommented, "1\n3\n4\n");
    }
    
    #[test]
    fn test_read_until_data_start() {
        let mut file = BufReader::new(File::open("tests/read_metadata_section").unwrap()).lines();
        let metadata_section = read_until_line_starts_with("Data:", &mut file).unwrap();
        assert_eq!(metadata_section, "Metadata:\n  - name: Stream1\n    dtype: int\n    shape: [1]\n");
        let next_line = file.next().unwrap().unwrap();
        assert_eq!(next_line, "1");
    }

    #[test]
}

