use std::{io::{BufReader, BufRead, Lines}, collections::HashMap};
// #[derive(Clone, Debug)]
struct Directory {
    parent: Option<String>,
    // name: String
}
// #[derive(Clone, Debug)]
struct File {
    parent_dir: String,
    // name: String,
    size: i32
}
// #[derive(Clone, Debug)]
struct ComputerDrive {
    cwd: Box<String>,
    // root_dir: Directory,
    directories: HashMap<String, Directory>,
    files: HashMap<String, File>
}

impl ComputerDrive {
    fn get_computer () -> Box<Self> {
        let mut comp = Box::new(
            ComputerDrive {
                cwd: Box::new("/".to_string()),
                // root_dir: Directory { parent: None, name: "/".to_string() },
                directories: HashMap::new(),
                files: HashMap::new()
            });
        // now we have the basis for our computer representation, we can build it up from the information we know
        for line in get_input() {
            match line {
                Ok(line) => {
                    // println!("{}", line);
                    // we can discern what command it is by looking at the first three
                    let first_three: &str = &line[..3];
                    match first_three {
                        "$ c" => {
                            // new directory
                            let next_dir = line.split_whitespace().last().unwrap();
                            comp.set_working_dir(next_dir.to_string());
                        },
                        "$ l" => {
                            // do nothing actually
                        },
                        "dir" => {
                            // found a new directory, add it to the Map
                            comp.mkdir(line.split_whitespace().last().unwrap().to_string());
                        },
                        _ => {
                            // this is a file, split on the whitespace, first part is size, second is name
                            let file_info = line.split_whitespace().collect::<Vec<&str>>();
                            let size = file_info.get(0).unwrap().parse::<i32>().unwrap();
                            let name = file_info.get(1).unwrap();
                            let full_name = *comp.cwd.to_owned() + name;
                            comp.files.insert(full_name, File {
                                parent_dir: *comp.cwd.to_owned(),
                                // name: name.to_string(),
                                size: size
                            });
                        }
                    }
                },
                _ => {
                    continue;
                }
            }
                
            
        }
        comp
    }

    fn set_working_dir(&mut self, dir: String) {
        let mut cwd: String = *(&self.cwd).to_owned();
        if dir == ".." {
            let mut dir_parts = cwd.split("/").filter(|p| p != &"").collect::<Vec<&str>>();
            if dir_parts.len() == 1 {
                // we are back at the root dir
                cwd = "/".to_string();
            } else if dir_parts.len() > 1 {
                // take off the last dir
                dir_parts.pop();
                cwd = "/".to_string() + &dir_parts.join("/").to_string() + "/";
            } else {
                panic!("Tried to cd .. from root dir");
            }
            
        } else if !dir.starts_with("/") {
            // relative dir, so append to cwd
            cwd = cwd + &dir + "/";
        } else {
            if dir.ends_with("/") {
                cwd = dir;
            } else {
                cwd = dir + "/"
            }
            
        }
        *self.cwd = cwd;
    }

    fn mkdir(&mut self, dir: String) {
        let new_dir = match dir.chars().nth(0).unwrap() {
            '/' => {
                dir.to_owned()
            },
            _ => {
                *self.cwd.to_owned() + &dir + "/"
            }
        };
        if !self.directories.contains_key(&new_dir) {
            if new_dir == "/" {
                // special case
                self.directories.insert(new_dir, Directory { parent: None, /* name: dir */ });
            } else {
                self.directories.insert(new_dir, Directory { parent: Some(*self.cwd.to_owned()), /* name: dir */  });
            }
            
        }
    }
}

fn get_input() -> Lines<BufReader<&'static [u8]>> {
    let input: Box<&[u8]> = Box::new(include_bytes!("input.txt"));
    BufReader::new(*input).lines()
}

fn get_dir_totals() -> HashMap<String, i32> {
    let comp = ComputerDrive::get_computer();
    
    let du: HashMap<String, i32> = comp.files.iter().fold(HashMap::new(), 
        |list, file_info| {
            // let file_name = file_info.0;
            let file = file_info.1;
            // inefficient but clone the list each time because otherwise we need to use a different way of looping over these files
            let mut new_list = list.clone();

            // for each of the files we want to add its size to it's parent dir and all of its parent dir's parent dirs
            // let mut parent_dir_parent_dir = comp.directories.get(&file.parent_dir).unwrap().parent;
            let mut key = Box::new(Some(file.parent_dir.to_owned().as_str().to_owned()));
            while key.is_some() {
                
                let dir_entry = new_list.entry(key.clone().unwrap());
                dir_entry.and_modify(|e| { *e += file.size }).or_insert(file.size);
                // parent_dir_parent_dir = comp.directories.get(key).unwrap().parent;
                let n = comp.directories.get(&key.unwrap());
                key = Box::new(n.map(|x| x.parent.to_owned().unwrap()));
            }
            new_list
        }
    ); // end fold

    du
}

pub fn run_part1() {
    
    let du = get_dir_totals();

    println!("{:?}", du);

    let final_result: i64 = du.iter().fold(0, |total, dir| { if *dir.1 < 100000 { total + *dir.1 as i64} else { total }});
    println!("Total: {}", final_result);
}

pub fn run_part2() {
    let du = get_dir_totals();

    let mut smallest_workable = ("".to_owned(), i32::max_value());
    let total_used: i32 = du.iter().fold(0, |total, dir| { if *dir.0 == "/" { *dir.1 } else { total } });
    let need_at_least = 30000000 - (70000000 - total_used);

    for d in du.iter() {
        if *d.1 < smallest_workable.1 && *d.1 > need_at_least {
            smallest_workable.0 = (*d.0).clone().to_string();
            smallest_workable.1 = *d.1;
        }
    }
    println!("Found {} in dir {}", smallest_workable.1, smallest_workable.0);
}