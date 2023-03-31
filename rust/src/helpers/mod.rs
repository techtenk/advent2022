use std::{io::{Lines, BufReader, BufRead, Read, Write}, fs::File};


pub fn get_input_lines<'a>(path: &str, mut array_buf: &'a mut[u8]) -> Lines<BufReader<&'a[u8]>> {
    let mut vec_buf = vec!['\n' as u8; 33];
    if let Ok(mut file) = File::open(path) {
        vec_buf.clear();
        let my_bytes = file.read_to_end(&mut vec_buf);
    }
    {
        let ref mut this: &mut [u8] = array_buf;
        let src: &[u8] = &(vec_buf.bytes().map(|x| x.unwrap()).collect::<Box<[u8]>>());
        // let res = (*this).write_all(src);
        // if res.is_err(){
        //     panic!("didn't work");
        // } else {
        //     let flush_res = this.flush();
        //     if flush_res.is_err() {
        //         panic!("{:?}", flush_res);
        //     }
        // }
        // I can't understand it, but rewind the array_buf pointer to original place
        let ptr: *mut u8 = array_buf.as_mut_ptr();
        unsafe {
            for i in 0..33 {
                std::ptr::write_bytes(ptr.offset(i), *src.as_ptr().offset(i), 33);
            }
            
        }
    };
    // array_buf.write(&vec_buf);
    // let buf: &mut dyn Read = &mut (array_buf.as_ref() as &[u8]) as &mut dyn Read;

    return BufReader::new(&(*array_buf)).lines();
}

