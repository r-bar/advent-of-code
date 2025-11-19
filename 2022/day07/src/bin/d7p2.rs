use anyhow::anyhow as e;
use anyhow::Context;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

const REQUIRED_SPACE: usize = 30000000;
const TOTAL_SPACE: usize = 70000000;

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
        match parts[..] {
            ["$", "ls"] => Ok(InputLine::Ls),
            ["$", "cd", dir] => Ok(InputLine::Cd(dir.to_string())),
            ["dir", name] => Ok(InputLine::Dir(name.to_string())),
            [size, name] => {
                let size = size.parse::<usize>()?;
                Ok(InputLine::File(size, name.to_string()))
            }
            _ => Err(e!("unknown command {}", s)),
        }
    }
}

enum FileSystem {
    Dir(String, HashMap<String, FileSystem>),
    File(String, usize),
}

impl FileSystem {
    fn name(&self) -> &str {
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

    fn children(&self) -> Option<&HashMap<String, Self>> {
        if let Self::Dir(_, children) = self {
            return Some(children);
        }
        None
    }

    fn children_mut(&mut self) -> Option<&mut HashMap<String, Self>> {
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
                self.cwd.pop().context("no parent directory")?;
            }
            _ => {
                self.fs
                    .get_path(&self.cwd)
                    .context("current working directory does not exist")?
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
    let free_space = TOTAL_SPACE - machine.fs.du();
    let need_freed = REQUIRED_SPACE - free_space;
    assert!(need_freed > 0, "we actually need to delete stuff right?");
    let mut candidates: Vec<_> = (&machine.fs)
        .into_iter()
        .filter(|fs| fs.is_dir() && fs.du() >= need_freed)
        .collect();
    candidates.sort_by_key(|fs| fs.du());
    let to_delete = candidates.first().context("no candidates found")?;
    println!("{}", to_delete.du());
    Ok(())
}
