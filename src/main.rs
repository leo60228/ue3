use omnom::ReadExt;
use std::env;
use std::fs::File;
use std::io::{self, prelude::*};

#[derive(Default, Debug, Clone)]
pub struct UnrealPackage {
    Tag: u32,
    FileVersion: u16,
    LicenseeVersion: u16,
    PackageFlags: i32,
    NameCount: i32,
    NameOffset: i32,
    ExportCount: i32,
    ExportOffset: i32,
    ImportCount: i32,
    ImportOffset: i32,
    Guid: [u8; 16],
    Generations: Vec<()>,
    HeadersSize: i32,
    PackageGroup: String,
    DependsOffset: i32,
    f38: i32,
    f3C: i32,
    f40: i32,
    EngineVersion: i32,
    CookerVersion: i32,
    CompressionFlags: i32,
    CompressedChunks: Vec<()>,
    U3unk60: i32,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct UnrealBlock {
    pub compressed_size: u32,
    pub uncompressed_size: u32,
}

const BLOCK_SIZE: usize = 0x20000;
/*
impl UnrealPackage {
    pub fn parse<R: Read>(r: &mut R) -> io::Result<Self> {
        let mut this: Self = Default::default();

        this.tag = r.read_le()?;
        let _unknown: u32 = r.read_le()?;
        this.summary = UnrealBlock::parse(r)?;

        let mut comp = 0;
        let mut uncomp = 0;
        while comp < this.summary.compressed_size && uncomp < this.summary.uncompressed_size {
            let block = UnrealBlock::parse(r)?;
            this.blocks.push(block);
            comp += block.compressed_size;
            uncomp += block.uncompressed_size;
        }

        assert_eq!(comp, this.summary.compressed_size);
        assert_eq!(uncomp, this.summary.uncompressed_size);

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

fn main() {
    let path = env::args().nth(1).unwrap();
    let mut file = File::open(path).unwrap();
    let package = UnrealPackage::parse(&mut file);
    println!("{:#?}", package);
}*/
fn main() {}
