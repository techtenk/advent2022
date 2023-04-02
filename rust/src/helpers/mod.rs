use std::{io::{Lines, BufReader, BufRead, Read}, fs::File};

#[macro_export]
macro_rules! get_file_path {
    ( $( $x:expr )? ) => {
        {
            let full_input_path = Path::new(file!()).parent().unwrap().as_os_str().to_str().unwrap().to_string() + "/" + $($x)?;
            full_input_path
        }
    };
}

pub fn get_input_lines<'a>(path: &str, vec_buffer: &'a mut Vec<u8>) -> Lines<BufReader<&'a[u8]>> {
    
    if let Ok(mut file) = File::open(path) {
        let my_bytes = file.read_to_end(vec_buffer);
        if my_bytes.is_err() {
            panic!("Could not read the file {}", path);
        }
    }

    return BufReader::new((*vec_buffer).as_slice()).lines();
}

