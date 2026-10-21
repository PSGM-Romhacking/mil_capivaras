//! Save states: a snapshot of the whole console and its file format.

use super::Pv1000;

/// Snapshot of the console at one instant (~33 KB): CPU, RAM 0x8000–0xFFFF, IO and sound.
pub struct Snapshot {
    pub(super) cpu: Vec<u8>,
    pub(super) ram: Vec<u8>,
    pub(super) io_regs: [u8; 8],
    pub(super) io_data: u8,
    pub(super) fd_data: u8,
    pub(super) fd_flag: bool,
    pub(super) snd_ch: [u8; 3],
    pub(super) force_pattern: bool,
    pub(super) pcg_bank: bool,
    pub(super) border: u8,
    /// Version 2: ports FF and FE and the hardware model's interrupts.
    pub(super) ff: u8,
    pub(super) fe: u8,
    pub(super) irq_enabled: u8,
    pub(super) irq_active: u8,
    /// Version 3: the matrix inputs' delay (hardware model).
    pub(super) matrix_held: u8,
    pub(super) matrix_released: [u64; 4],
    pub(super) period: [u8; 3],
    pub(super) tone: [bool; 3],
    pub(super) next_step: u64,
    pub(super) next_sample: f64,
    pub(super) cpu_t: u64,
    pub(super) line_t: u64,
}

const STATE_MAGIC: &[u8; 4] = b"PVST";
const STATE_VERSION: u8 = 3;

impl Snapshot {
    /// Save state file: "PVST", version, ROM fingerprint and the state.
    pub fn to_bytes(&self, rom_hash: u64) -> Vec<u8> {
        let mut v = Vec::with_capacity(self.ram.len() + 128);
        v.extend_from_slice(STATE_MAGIC);
        v.push(STATE_VERSION);
        v.extend_from_slice(&rom_hash.to_le_bytes());
        v.extend_from_slice(&(self.cpu.len() as u32).to_le_bytes());
        v.extend_from_slice(&self.cpu);
        v.extend_from_slice(&self.ram);
        v.extend_from_slice(&self.io_regs);
        v.extend_from_slice(&[self.io_data, self.fd_data, self.fd_flag as u8]);
        v.extend_from_slice(&self.snd_ch);
        v.extend_from_slice(&[self.force_pattern as u8, self.pcg_bank as u8, self.border]);
        v.extend_from_slice(&self.period);
        v.extend(self.tone.iter().map(|&t| t as u8));
        v.extend_from_slice(&self.next_step.to_le_bytes());
        v.extend_from_slice(&self.next_sample.to_le_bytes());
        v.extend_from_slice(&self.cpu_t.to_le_bytes());
        v.extend_from_slice(&self.line_t.to_le_bytes());
        v.extend_from_slice(&[self.ff, self.fe, self.irq_enabled, self.irq_active]);
        v.push(self.matrix_held);
        for r in self.matrix_released {
            v.extend_from_slice(&r.to_le_bytes());
        }
        v
    }

    pub fn from_bytes(b: &[u8], rom_hash: u64) -> Result<Self, String> {
        Self::parse(b, Some(rom_hash))
    }

    /// Reads without checking the ROM hash (the caller already checked it, e.g. the frontend).
    pub fn decode(b: &[u8]) -> Result<Self, String> {
        Self::parse(b, None)
    }

    fn parse(b: &[u8], rom_hash: Option<u64>) -> Result<Self, String> {
        let mut r = Reader { b, i: 0 };
        if r.take(4)? != STATE_MAGIC {
            return Err("not a PV-1000 save state".into());
        }
        let version = r.u8()?;
        if !(1..=STATE_VERSION).contains(&version) {
            return Err("incompatible save state version".into());
        }
        let h = r.u64()?;
        if rom_hash.is_some_and(|x| x != h) {
            return Err("this state is from another game (different ROM)".into());
        }
        let n = u32::from_le_bytes(r.take(4)?.try_into().unwrap()) as usize;
        let cpu = r.take(n)?.to_vec();
        let ram = r.take(0x8000)?.to_vec();
        let io_regs = r.take(8)?.try_into().unwrap();
        let (io_data, fd_data, fd_flag) = (r.u8()?, r.u8()?, r.u8()? != 0);
        let snd_ch = r.take(3)?.try_into().unwrap();
        let (force_pattern, pcg_bank, border) = (r.u8()? != 0, r.u8()? != 0, r.u8()?);
        let period = r.take(3)?.try_into().unwrap();
        let t = r.take(3)?;
        let tone = [t[0] != 0, t[1] != 0, t[2] != 0];
        let next_step = r.u64()?;
        let next_sample = f64::from_le_bytes(r.take(8)?.try_into().unwrap());
        let (cpu_t, line_t) = (r.u64()?, r.u64()?);
        // version 1 had only the MiSTer core's bits of port FF, and no FE or interrupts
        let (ff, fe, irq_enabled, irq_active) = if version >= 2 {
            (r.u8()?, r.u8()?, r.u8()?, r.u8()?)
        } else {
            ((force_pattern as u8) << 4 | (pcg_bank as u8) << 5 | border, 0xB8, 0, 0)
        };
        let (mut matrix_held, mut matrix_released) = (0, [0; 4]);
        if version >= 3 {
            matrix_held = r.u8()?;
            for m in &mut matrix_released {
                *m = r.u64()?;
            }
        }
        Ok(Self { cpu, ram, io_regs, io_data, fd_data, fd_flag, snd_ch, force_pattern, pcg_bank, border, ff, fe, irq_enabled, irq_active, matrix_held, matrix_released, period, tone, next_step, next_sample, cpu_t, line_t })
    }
}

struct Reader<'a> {
    b: &'a [u8],
    i: usize,
}

impl<'a> Reader<'a> {
    fn take(&mut self, n: usize) -> Result<&'a [u8], String> {
        let s = self.b.get(self.i..self.i + n).ok_or("incomplete state file")?;
        self.i += n;
        Ok(s)
    }
    fn u8(&mut self) -> Result<u8, String> {
        Ok(self.take(1)?[0])
    }
    fn u64(&mut self) -> Result<u64, String> {
        Ok(u64::from_le_bytes(self.take(8)?.try_into().unwrap()))
    }
}

/// ROM fingerprint (FNV-1a 64), so a state from another game is not loaded.
pub fn rom_hash(rom: &[u8]) -> u64 {
    rom.iter().fold(0xcbf2_9ce4_8422_2325u64, |h, &b| (h ^ b as u64).wrapping_mul(0x0000_0100_0000_01B3))
}

impl Pv1000 {
    /// Full state (except the ROM, which does not change) for rewinding.
    pub fn save(&self) -> Snapshot {
        let b = &self.bus;
        let snd = &b.sound;
        Snapshot {
            cpu: self.cpu.serialize(),
            ram: b.mem[0x8000..].to_vec(),
            io_regs: b.io_regs,
            io_data: b.io_data,
            fd_data: b.fd_data,
            fd_flag: b.fd_flag,
            snd_ch: b.snd_ch,
            force_pattern: b.force_pattern,
            pcg_bank: b.pcg_bank,
            border: b.border,
            ff: b.ff,
            fe: b.fe,
            irq_enabled: b.irq_enabled,
            irq_active: b.irq_active,
            matrix_held: b.matrix_held,
            matrix_released: b.matrix_released,
            period: snd.period,
            tone: snd.tone,
            next_step: snd.next_step,
            next_sample: snd.next_sample,
            cpu_t: self.cpu_t,
            line_t: self.line_t,
        }
    }

    pub fn restore(&mut self, st: &Snapshot) {
        let _ = self.cpu.deserialize(&st.cpu);
        let b = &mut self.bus;
        b.mem[0x8000..].copy_from_slice(&st.ram);
        b.io_regs = st.io_regs;
        b.io_data = st.io_data;
        b.fd_data = st.fd_data;
        b.fd_flag = st.fd_flag;
        b.snd_ch = st.snd_ch;
        b.force_pattern = st.force_pattern;
        b.pcg_bank = st.pcg_bank;
        b.border = st.border;
        b.ff = st.ff;
        b.fe = st.fe;
        b.irq_enabled = st.irq_enabled;
        b.irq_active = st.irq_active;
        b.matrix_held = st.matrix_held;
        b.matrix_released = st.matrix_released;
        b.sound.period = st.period;
        b.sound.tone = st.tone;
        b.sound.next_step = st.next_step;
        b.sound.next_sample = st.next_sample;
        b.sound.out.clear();
        self.cpu_t = st.cpu_t;
        self.line_t = st.line_t;
    }
}
