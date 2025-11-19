use anyhow::anyhow as e;
use anyhow::Context;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

type Path = Vec<String>;

enum InputLine {
    Cd(String),
    Ls,
    File(usize, String),
    Dir(String),
}

impl FromStr for InputLine {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_end();
        let parts: Vec<&str> = s.split(|c: char| c.is_whitespace()).collect();
        assert!(parts.len() > 1);
        if s.starts_with("$ cd") {
            let dir = parts.last().ok_or_else(|| e!("malformed command {}", s))?;
            return Ok(InputLine::Cd(dir.to_string()));
        }
        if s == "$ ls" {
            return Ok(InputLine::Ls);
        }
        if parts[0] == "dir" {
            let dir = parts.last().ok_or_else(|| e!("malformed command {}", s))?;
            return Ok(InputLine::Dir(dir.to_string()));
        }
        if let Ok(size) = parts[0].parse::<usize>() {
            return Ok(InputLine::File(size, parts[1].to_string()));
        }
        Err(e!("unknown command {}", s))
    }
}

enum FileSystem {
    Dir(String, HashMap<String, FileSystem>),
    File(String, usize),
}

impl FileSystem {
    fn name<'a>(&'a self) -> &'a str {
        match self {
            Self::Dir(name, _) => name,
            Self::File(name, _) => name,
        }
    }

    fn get<'a>(&'a self, name: &str) -> Option<&'a Self> {
        if let Self::Dir(_, children) = self {
            return children.get(name);
        }
        None
    }

    fn get_mut<'a>(&'a mut self, name: &str) -> Option<&'a mut Self> {
        if let Self::Dir(_, children) = self {
            return children.get_mut(name);
        }
        None
    }

    fn get_path<'a>(&'a self, path: &Path) -> Option<&'a Self> {
        let path = if path.first() == Some(&"".to_string()) {
            &path[1..]
        } else {
            &path[..]
        };
        let mut ptr = self;
        for part in path {
            match ptr.get(part) {
                Some(fs) => ptr = fs,
                None => return None,
            }
        }
        Some(ptr)
    }

    fn get_path_mut<'a>(&'a mut self, path: &Path) -> Option<&'a mut Self> {
        let path = if path.first() == Some(&"".to_string()) {
            &path[1..]
        } else {
            &path[..]
        };
        let mut ptr = self;
        for part in path {
            match ptr.get_mut(part) {
                Some(fs) => ptr = fs,
                None => return None,
            }
        }
        Some(ptr)
    }

    fn insert(&mut self, parent_path: &Path, fs: FileSystem) -> anyhow::Result<()> {
        let parent = self
            .get_path_mut(parent_path)
            .context("path does not exist")?;
        let children = parent.children_mut().context("path is not a directory")?;
        children.insert(fs.name().to_string(), fs);
        Ok(())
    }

    fn children<'a>(&'a self) -> Option<&'a HashMap<String, Self>> {
        if let Self::Dir(_, children) = self {
            return Some(children);
        }
        None
    }

    fn children_mut<'a>(&'a mut self) -> Option<&'a mut HashMap<String, Self>> {
        if let Self::Dir(_, children) = self {
            return Some(children);
        }
        None
    }

    fn is_dir(&self) -> bool {
        if let FileSystem::Dir(_, _) = self {
            return true;
        }
        false
    }
}

impl<'a> IntoIterator for &'a FileSystem {
    type Item = &'a FileSystem;
    type IntoIter = FileSystemIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        FileSystemIterator(vec![self])
    }
}

struct FileSystemIterator<'a>(Vec<&'a FileSystem>);

impl<'a> Iterator for FileSystemIterator<'a> {
    type Item = &'a FileSystem;

    fn next<'b>(&mut self) -> Option<Self::Item> {
        //todo!()
        let n = self.0.pop();
        match n {
            Some(FileSystem::File(_, _)) => n,
            Some(FileSystem::Dir(_, children)) => {
                self.0.extend(children.values());
                n
            }
            None => None,
        }
    }
}

struct FileSystemMutIterator<'a>(Vec<&'a mut FileSystem>);

//impl<'a> Iterator for FileSystemMutIterator<'a> {
//    type Item = &'a mut FileSystem;

//    fn next<'b>(&mut self) -> Option<Self::Item> {
//        //todo!()
//        let n = self.0.pop();
//        match n {
//            Some(FileSystem::File(_, _)) => n,
//            Some(FileSystem::Dir(_, children)) => {
//                self.0.extend(children.values_mut());
//                n
//            }
//            None => None,
//        }
//    }
//}

impl FileSystem {
    fn du(&self) -> usize {
        let mut size = 0;
        for fs in self {
            if let FileSystem::File(_, file_size) = fs {
                size += file_size;
            }
        }
        size
    }
}

struct Machine {
    fs: FileSystem,
    cwd: Path,
}

impl Machine {
    fn new() -> Self {
        Machine {
            cwd: Vec::new(),
            fs: FileSystem::Dir("".to_string(), HashMap::new()),
        }
    }

    fn run(&mut self, cmd: &InputLine) -> anyhow::Result<()> {
        match cmd {
            InputLine::Ls => Ok(()),
            InputLine::Cd(dir) => {
                if dir == "/" {
                    Ok(())
                } else {
                    self.cd(dir)
                }
            }
            InputLine::Dir(name) => self
                .fs
                .insert(&self.cwd, FileSystem::Dir(name.clone(), HashMap::new())),
            InputLine::File(size, name) => self
                .fs
                .insert(&self.cwd, FileSystem::File(name.clone(), *size)),
        }
    }

    fn cd(&mut self, dir: &str) -> anyhow::Result<()> {
        match dir {
            ".." => {
                //self.cwd.pop().ok_or_else(|| e!("no parent directory"))?;
                self.cwd.pop().context("no parent directory")?;
            }
            _ => {
                self.fs
                    .get_path(&self.cwd)
                    .ok_or_else(|| e!("current working directory does not exist"))?
                    .get(dir);
                self.cwd.push(dir.to_string());
            }
        }
        Ok(())
    }

    fn pwd(&self) -> String {
        let mut s = self.cwd.join("/");
        s.insert(0, '/');
        s
    }
}

fn main() -> anyhow::Result<()> {
    let filename = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let f = File::open(&filename)?;
    let reader = BufReader::new(f);
    let mut machine = Machine::new();
    for line in reader.lines() {
        let cmd: InputLine = line?.parse()?;
        machine.run(&cmd)?;
    }
    let total: usize = (&machine.fs)
        .into_iter()
        .filter(|fs| fs.is_dir())
        .inspect(|fs| {
            println!("{} {}", fs.name(), fs.du());
        })
        .map(|dir| dir.du())
        .filter(|&size| size < 100000)
        .sum();
    println!("{}", total);
    Ok(())
}
