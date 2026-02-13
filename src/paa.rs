// author: fixitdaztv
// date: 2026-02-13
use binrw::{BinRead, BinResult, Endian};
use std::io::SeekFrom;

#[derive(BinRead, Debug, Clone, Copy, PartialEq)]
#[br(repr = u16)]
pub enum PaaType {
    Dxt1 = 0xFF01,
    Dxt2 = 0xFF02,
    Dxt3 = 0xFF03,
    Dxt4 = 0xFF04,
    Dxt5 = 0xFF05,
    Argb4444 = 0x4444,
    Argb1555 = 0x1555,
    Argb8888 = 0x8888,
}

#[derive(BinRead, Debug)]
pub struct PaaTagg {
    pub signature: [u8; 4],
    pub name: [u8; 4],
    pub data_len: u32,
    #[br(count = data_len)]
    pub data: Vec<u8>,
}

#[derive(BinRead, Debug)]
pub struct Mipmap {
    pub raw_width: u16,
    pub raw_height: u16,
    #[br(parse_with = read_u24)]
    pub data_len: u32,
    #[br(count = data_len)]
    pub data: Vec<u8>,
}

impl Mipmap {
    pub fn width(&self) -> u16 { self.raw_width & 0x7FFF }
    pub fn height(&self) -> u16 { self.raw_height & 0x7FFF }
}

fn read_u24<R: binrw::io::Read + binrw::io::Seek>(
    reader: &mut R,
    _endian: Endian,
    _: (),
) -> BinResult<u32> {
    let mut buf = [0u8; 3];
    reader.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes([buf[0], buf[1], buf[2], 0]))
}

#[derive(BinRead, Debug)]
#[br(little)]
pub struct PaaFile {
    pub type_tag: PaaType,
    #[br(parse_with = read_taggs)]
    pub taggs: Vec<PaaTagg>,
    #[br(parse_with = read_mipmaps)]
    pub mipmaps: Vec<Mipmap>,
}

fn read_taggs<R: binrw::io::Read + binrw::io::Seek>(
    reader: &mut R,
    endian: Endian,
    _: (),
) -> BinResult<Vec<PaaTagg>> {
    let mut taggs = Vec::new();
    loop {
        let pos = reader.stream_position()?;
        let signature = match <[u8; 4]>::read_options(reader, endian, ()) {
            Ok(sig) => sig,
            Err(_) => break,
        };

        if &signature == b"GGAT" {
            reader.seek(SeekFrom::Start(pos))?;
            taggs.push(PaaTagg::read_options(reader, endian, ())?);
        } else {
            reader.seek(SeekFrom::Start(pos))?;
            break;
        }
    }
    Ok(taggs)
}

fn read_mipmaps<R: binrw::io::Read + binrw::io::Seek>(
    reader: &mut R,
    endian: Endian,
    _: (),
) -> BinResult<Vec<Mipmap>> {
    let mut mipmaps = Vec::new();
    
    // Scan forward until we find something that looks like a valid Mipmap (width/height > 0)
    loop {
        let pos = reader.stream_position()?;
        let res = Mipmap::read_options(reader, endian, ());
        
        match res {
            Ok(mip) => {
                let w = mip.width();
                let h = mip.height();
                // Valid power-of-two or standard texture check
                if w > 0 && h > 0 && w <= 8192 && h <= 8192 && (w.is_power_of_two() || w % 4 == 0) {
                    mipmaps.push(mip);
                    break; 
                } else {
                    reader.seek(SeekFrom::Start(pos + 1))?;
                }
            }
            Err(_) => {
                if reader.seek(SeekFrom::Start(pos + 1)).is_err() { break; }
            }
        }
    }

    // Continue reading subsequent mipmaps normally
    loop {
        match Mipmap::read_options(reader, endian, ()) {
            Ok(mip) => {
                if mip.width() == 0 || mip.height() == 0 { break; }
                mipmaps.push(mip);
            }
            Err(_) => break,
        }
    }
    Ok(mipmaps)
}