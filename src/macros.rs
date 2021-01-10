#[macro_export]
macro_rules! readfile {
    ( $filename:expr ) => {{
        use std::fs::File;
        use std::io::{BufRead, BufReader};
        let f = File::open($filename).unwrap();
        let reader = BufReader::new(f);
        let lines = reader.lines().collect::<Vec<_>>();
        lines
            .into_iter()
            .map(|line| line.unwrap())
            .collect::<Vec<String>>()
    }};
}
