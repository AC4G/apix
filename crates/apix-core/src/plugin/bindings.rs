use std::{fs::File, io::{Read, Seek, SeekFrom, Write}};

use mlua::UserData;

pub struct LuaFile(pub File);

impl UserData for LuaFile {
    fn add_methods<'lua, M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("read_all", |_, this, ()| {
            let mut file = &this.0;
            let mut buf = String::new();
            file.seek(SeekFrom::Start(0))?;
            file.read_to_string(&mut buf)?;
            Ok(buf)
        });

        methods.add_method("read_bytes", |_, this, (offset, length): (u64, usize)| {
            let mut file = &this.0;
            let mut buf = vec![0u8; length];
            file.seek(SeekFrom::Start(offset))?;
            let read_bytes = file.read(&mut buf)?;
            buf.truncate(read_bytes);
            Ok(buf)
        });

        methods.add_method("write_at", |_, this, (offset, data): (u64, mlua::String)| {
            let mut file = &this.0;
            file.seek(SeekFrom::Start(offset))?;
            file.write_all(&data.as_bytes())?;
            Ok(())
        });
    }
}
