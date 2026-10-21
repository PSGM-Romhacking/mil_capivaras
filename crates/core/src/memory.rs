//! PV-1000 memory map and "flat" reads, in the format RetroAchievements
//! (rcheevos) uses: achievements point to addresses from 0 to `FLAT_SIZE - 1`,
//! which the emulator translates to CPU addresses.
//!
//! Measured on the 13 cartridges and the homebrews: they use 0xB800–0xBFFF (2 KB), the
//! console's real RAM. The rest of 0x8000–0xFFFF exists in the core's array; only Battlot
//! writes there (above 0xDC00, see `machine/bus.rs`).
#![allow(dead_code)] // groundwork for RetroAchievements: used once rcheevos is added

use crate::machine::Pv1000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Rom,
    VideoRam,
    Ram,
}

#[derive(Clone, Copy, Debug)]
pub struct Region {
    pub name: &'static str,
    pub cpu_start: u16,
    pub len: u32,
    pub kind: Kind,
    /// Start in the flat space; `None` = outside achievements (ROM).
    pub flat_start: Option<u32>,
}

pub const MAP: &[Region] = &[
    Region { name: "Cartridge (ROM)", cpu_start: 0x0000, len: 0x8000, kind: Kind::Rom, flat_start: None },
    Region { name: "Screen: tile names (32x24)", cpu_start: 0xB800, len: 0x300, kind: Kind::VideoRam, flat_start: Some(0x000) },
    Region { name: "Work RAM", cpu_start: 0xBB00, len: 0x100, kind: Kind::Ram, flat_start: Some(0x300) },
    Region { name: "PCG / RAM (patterns of tiles >= 0xE0 or variables)", cpu_start: 0xBC00, len: 0x400, kind: Kind::Ram, flat_start: Some(0x400) },
];

/// Size of the flat space (2 KB).
pub const FLAT_SIZE: u32 = 0x800;

pub fn flat_to_cpu(addr: u32) -> Option<u16> {
    MAP.iter().find_map(|r| {
        let f = r.flat_start?;
        (addr >= f && addr < f + r.len).then(|| r.cpu_start + (addr - f) as u16)
    })
}

pub fn cpu_to_flat(addr: u16) -> Option<u32> {
    MAP.iter().find_map(|r| {
        let f = r.flat_start?;
        let a = addr as u32;
        let s = r.cpu_start as u32;
        (a >= s && a < s + r.len).then(|| f + (a - s))
    })
}

/// Reads `buf.len()` bytes starting at the flat address; returns how many were read (the
/// rcheevos `read_memory` format: outside the map, reads 0 bytes).
pub fn read_flat(pv: &Pv1000, addr: u32, buf: &mut [u8]) -> usize {
    let mem = pv.ram();
    let mut n = 0;
    for (i, b) in buf.iter_mut().enumerate() {
        let Some(cpu) = flat_to_cpu(addr + i as u32) else { break };
        *b = mem[cpu as usize];
        n += 1;
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flat_space_covers_whole_ram_without_holes() {
        for a in 0..FLAT_SIZE {
            let cpu = flat_to_cpu(a).expect("hole in the map");
            assert!((0xB800..=0xBFFF).contains(&cpu));
            assert_eq!(cpu_to_flat(cpu), Some(a));
        }
        assert_eq!(flat_to_cpu(FLAT_SIZE), None);
        assert_eq!(cpu_to_flat(0x1234), None); // ROM is left out
    }

    #[test]
    fn flat_read_crosses_regions() {
        let mut pv = Pv1000::new(48000);
        let mut rom = vec![0u8; 0x100];
        // program: writes 0xAA to 0xBAFF and 0xBB to 0xBB00 (region boundary) and halts
        rom[..12].copy_from_slice(&[0x3E, 0xAA, 0x32, 0xFF, 0xBA, 0x3E, 0xBB, 0x32, 0x00, 0xBB, 0x76, 0x00]);
        pv.load(&rom);
        pv.run_frame();
        let mut buf = [0u8; 2];
        assert_eq!(read_flat(&pv, 0x2FF, &mut buf), 2);
        assert_eq!(buf, [0xAA, 0xBB]);
        assert_eq!(read_flat(&pv, FLAT_SIZE - 1, &mut buf), 1); // runs past the end: reads only 1
    }
}
