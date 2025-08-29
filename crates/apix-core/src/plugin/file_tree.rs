use std::{
    fs,
    io::{self, Read},
    path::{Path, PathBuf},
};

use mlua::{UserData, UserDataMethods};

#[derive(Debug, Clone)]
pub struct LuaFile {
    pub path: PathBuf,
}

impl LuaFile {
    pub fn read(&self) -> io::Result<String> {
        let mut file = fs::File::open(&self.path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }
}

#[derive(Debug, Clone)]
pub struct LuaDir {
    pub path: PathBuf,
    pub entries: Vec<LuaEntry>,
}

#[derive(Debug, Clone)]
pub enum LuaEntry {
    File(LuaFile),
    Dir(LuaDir),
}

impl LuaDir {
    pub fn scan(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let mut entries = Vec::new();

        for entry in fs::read_dir(&path)? {
            let entry = entry?;
            let entry_path = entry.path();
            if entry.file_type()?.is_dir() {
                entries.push(LuaEntry::Dir(LuaDir::scan(&entry_path)?));
            } else {
                entries.push(LuaEntry::File(LuaFile { path: entry_path }));
            }
        }

        Ok(LuaDir { path, entries })
    }
}

impl UserData for LuaFile {
    fn add_methods<'lua, M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("read", |_, this, ()| match this.read() {
            Ok(content) => Ok(content),
            Err(e) => Err(mlua::Error::external(e)),
        });
        methods.add_method("path", |_, this, ()| {
            Ok(this.path.to_string_lossy().to_string())
        });
    }
}

impl UserData for LuaDir {
    fn add_methods<'lua, M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("entries", |lua, this, ()| {
            let table = lua.create_table()?;
            for (i, entry) in this.entries.iter().enumerate() {
                match entry {
                    LuaEntry::File(f) => table.set(i + 1, f.clone())?,
                    LuaEntry::Dir(d) => table.set(i + 1, d.clone())?,
                }
            }
            Ok(table)
        });
        methods.add_method("path", |_, this, ()| {
            Ok(this.path.to_string_lossy().to_string())
        });
    }

    fn register(registry: &mut mlua::UserDataRegistry<Self>) {
        Self::add_fields(registry);
        Self::add_methods(registry);
    }
}
