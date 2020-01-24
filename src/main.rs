use omnom::ReadExt;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::{self, prelude::*, SeekFrom};

#[derive(Default, Debug, Clone)]
pub struct UnrealPackage {
    pub tag: u32,
    pub file_version: u16,
    pub licensee_version: u16,
    pub package_flags: i32,
    pub names: Vec<UnrealName>,
    pub exports: Vec<UnrealExport>,
    pub imports: Vec<UnrealImport>,
    pub guid: [u8; 16],
    pub generations: Vec<UnrealGeneration>,
    pub headers_size: i32,
    pub package_group: String,
    pub depends_offset: i32,
    pub f38: i32,
    pub f3_c: i32,
    pub f40: i32,
    pub engine_version: i32,
    pub cooker_version: i32,
    pub compression_flags: i32,
    pub compressed_chunks: Vec<()>,
    pub u3unk60: i32,
    pub rest: Vec<u8>,
}

#[derive(Default, Debug, Clone)]
pub struct UnrealName(pub String, pub u64);

fn get_name<'a, R: Read>(r: &mut R, table: &'a [UnrealName]) -> io::Result<&'a str> {
    let idx: u32 = r.read_le()?;
    let _extra: u32 = r.read_le()?;
    Ok(&table[idx as usize].0)
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct UnrealImport {
    pub class_package: String,
    pub class_name: String,
    pub package_idx: u32,
    pub object_name: String,
}

#[derive(Default, Debug, Clone)]
pub struct UnrealExport {
    pub class_idx: u32,
    pub package_idx: u32,
    pub object_name: String,
    pub serial_size: u32,
    pub serial_offset: u32,
    pub super_idx: u32,
    pub object_flags: u64,
    pub export_flags: u32,
    pub archetype: i32,
    pub generations: Vec<u32>,
    pub guid: [u8; 16],
    pub package_flags: u32,
    pub unk_6c: u32,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct UnrealGeneration {
    pub export_count: i32,
    pub name_count: i32,
    pub net_object_count: i32,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct UnrealBlock {
    pub compressed_size: u32,
    pub uncompressed_size: u32,
}

fn parse_string<R: Read>(r: &mut R) -> io::Result<String> {
    let len: u32 = r.read_le()?;
    let mut data = Vec::new();
    data.resize(len as usize, 0);
    r.read_exact(&mut data)?;
    if data.last() == Some(&0) {
        data.pop();
    }
    Ok(String::from_utf8(data).unwrap())
}

impl UnrealPackage {
    pub fn parse<R: Read + Seek>(r: &mut R) -> io::Result<Self> {
        let mut this: Self = Default::default();

        this.tag = r.read_le()?;
        let version: u32 = r.read_le()?;
        this.file_version = (version & 0xFFFF) as u16;
        this.licensee_version = (version >> 16) as u16;
        if this.file_version >= 249 {
            this.headers_size = r.read_le()?;
        }
        if this.file_version >= 269 {
            this.package_group = parse_string(r)?;
        }
        this.package_flags = r.read_le()?;
        let name_count: u32 = r.read_le()?;
        let name_offset: u32 = r.read_le()?;
        let export_count: u32 = r.read_le()?;
        let export_offset: u32 = r.read_le()?;
        let import_count: u32 = r.read_le()?;
        let import_offset: u32 = r.read_le()?;
        if this.file_version >= 415 {
            this.depends_offset = r.read_le()?;
        }
        if this.file_version >= 623 {
            this.f38 = r.read_le()?;
            this.f3_c = r.read_le()?;
            this.f40 = r.read_le()?;
        }
        if this.file_version >= 584 {
            let _unknown: i32 = r.read_le()?;
        }
        r.read_exact(&mut this.guid)?;
        let generations: i32 = r.read_le()?;
        for _ in 0..generations {
            this.generations
                .push(UnrealGeneration::parse(r, this.file_version >= 322)?);
        }
        if this.file_version >= 245 {
            this.engine_version = r.read_le()?;
        }
        if this.file_version >= 277 {
            this.cooker_version = r.read_le()?;
        }
        if this.file_version >= 334 {
            this.compression_flags = r.read_le()?;
            // TODO: compression
        }
        if this.file_version >= 482 {
            this.u3unk60 = r.read_le()?;
        }
        let mut last = r.seek(SeekFrom::Current(0))?;
        if name_count > 0 {
            r.seek(SeekFrom::Start(name_offset.into()))?;
            for _ in 0..name_count {
                let s = parse_string(r)?;
                let f = r.read_le()?;
                this.names.push(UnrealName(s, f));
            }
            last = last.max(r.seek(SeekFrom::Current(0))?);
        }
        if import_count > 0 {
            r.seek(SeekFrom::Start(import_offset.into()))?;
            for _ in 0..import_count {
                this.imports.push(UnrealImport::parse(r, &this.names)?);
            }
            last = last.max(r.seek(SeekFrom::Current(0))?);
        }
        if export_count > 0 {
            r.seek(SeekFrom::Start(export_offset.into()))?;
            for _ in 0..export_count {
                this.exports.push(UnrealExport::parse(r, &this.names)?);
            }
            last = last.max(r.seek(SeekFrom::Current(0))?);
        }
        r.seek(SeekFrom::Start(last))?;
        r.read_to_end(&mut this.rest)?;
        Ok(this)
    }
}

impl UnrealBlock {
    pub fn parse<R: Read>(r: &mut R) -> io::Result<Self> {
        let mut this: Self = Default::default();

        this.compressed_size = r.read_le()?;
        this.uncompressed_size = r.read_le()?;

        Ok(this)
    }
}

impl UnrealImport {
    pub fn parse<R: Read>(r: &mut R, names: &[UnrealName]) -> io::Result<Self> {
        let mut this: Self = Default::default();

        this.class_package = get_name(r, names)?.to_string();
        this.class_name = get_name(r, names)?.to_string();
        this.package_idx = r.read_le()?;
        this.object_name = get_name(r, names)?.to_string();

        Ok(this)
    }
}

impl UnrealExport {
    pub fn parse<R: Read>(r: &mut R, names: &[UnrealName]) -> io::Result<Self> {
        let mut this: Self = Default::default();

        this.class_idx = r.read_le()?;
        this.super_idx = r.read_le()?;
        this.package_idx = r.read_le()?;
        this.object_name = get_name(r, names)?.to_string();
        this.archetype = r.read_le()?;
        this.object_flags = r.read_le()?;
        this.serial_size = r.read_le()?;
        this.serial_offset = r.read_le()?;
        this.export_flags = r.read_le()?;
        let len: u32 = r.read_le()?;
        for _ in 0..len {
            this.generations.push(r.read_le()?);
        }
        r.read_exact(&mut this.guid)?;
        this.unk_6c = r.read_le()?;

        Ok(this)
    }
}

impl UnrealGeneration {
    pub fn parse<R: Read>(r: &mut R, net_objects: bool) -> io::Result<Self> {
        let mut this: Self = Default::default();

        this.export_count = r.read_le()?;
        this.name_count = r.read_le()?;
        if net_objects {
            this.net_object_count = r.read_le()?;
        }

        Ok(this)
    }
}

fn main() {
    let path = env::args().nth(1).unwrap();
    let mut file = File::open(path).unwrap();
    let package = UnrealPackage::parse(&mut file).unwrap();
    serde_yaml::to_writer(io::stdout(), &package.imports).unwrap();
    io::stdout().flush().unwrap();
}
