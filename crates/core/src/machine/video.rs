//! Drawing: the current line is rendered as the CPU runs and redrawn when a write
//! changes what the video chip reads.

use super::*;

impl Pv1000 {
    /// Redraws the frame from the current state (after loading a state).
    pub fn redraw(&mut self) {
        for vc in 0..H as u32 {
            self.render_line(vc);
        }
    }

    /// The instruction at `pc` just executed (starting at `t0`, `dc` cycles) wrote to
    /// what the video reads: draw the current line up to each write with the values
    /// before it. Memory writes are an instruction's last machine cycles (3 cycles
    /// each, the last one ending about 2 cycles before the instruction does), except
    /// in LDI/LDD/LDIR/LDDR, which write at cycle 12 (fetch ED, fetch, read, write;
    /// the repeat's 5 extra cycles come after).
    pub(super) fn replay_video_writes(&mut self, pc: u16, t0: u64, dc: u64) {
        let writes = std::mem::take(&mut self.bus.video_writes);
        for &w in writes.iter().rev() {
            self.bus.apply(w, true);
        }
        let m = &self.bus.mem;
        let block = m[pc as usize] == 0xED && matches!(m[pc.wrapping_add(1) as usize], 0xA0 | 0xA8 | 0xB0 | 0xB8);
        let n = writes.len() as u64;
        for (k, &w) in writes.iter().enumerate() {
            let cycle = if block { 12 } else { dc.saturating_sub(2 + 3 * (n - 1 - k as u64)) };
            self.draw_until(t0 + cycle * U_CPU);
            self.bus.apply(w, false);
        }
        self.bus.video_writes = writes;
        self.bus.video_writes.clear(); // keeps the allocation
    }

    /// Draws the pixels of the current line sampled up to time `t`: column c is
    /// sampled when hc becomes 17 + c.
    pub(super) fn draw_until(&mut self, t: u64) {
        if self.line_vc as usize >= H || t < self.line_t {
            return;
        }
        let upto = if self.bus.hw {
            (((t - self.line_t) / HW_U_PX) as usize + 1).saturating_sub(HW_PICTURE_PX as usize).min(W)
        } else {
            (((t - self.line_t) / U_VDP) as usize + 1).saturating_sub(17).min(W)
        };
        if upto > self.drawn {
            self.render_span(self.line_vc, self.drawn, upto);
            self.drawn = upto;
        }
    }

    pub(super) fn render_line(&mut self, vc: u32) {
        self.render_span(vc, 0, W);
    }

    /// Same computation as the core's video state machine: 8x8 tiles at 0xB800,
    /// patterns in ROM (or PCG at 0xBC00 for tiles >= 0xE0), 3 R/G/B bitplanes.
    /// Draws columns `from..to` of line `vc` with the current memory and port FF.
    pub(super) fn render_span(&mut self, vc: u32, from: usize, to: usize) {
        let y = vc as i32 - 16;
        let row = &mut self.fb[vc as usize * W + from..vc as usize * W + to];
        let b = &self.bus;
        if !(0..192).contains(&y) || b.display_off() {
            let bc = b.border;
            let c = (if bc & 1 != 0 { 0xFF0000 } else { 0 }) | (if bc & 2 != 0 { 0x00FF00 } else { 0 }) | (if bc & 4 != 0 { 0x0000FF } else { 0 });
            row.fill(c);
            if to == W && !b.hw {
                row[to - from - 1] = 0; // HBlank starts at hc 240: the last column is never drawn
            }
            return;
        }
        let m = &b.mem;
        let y = y as usize;
        let line = y & 7;
        let mut cached: Option<(usize, [u8; 3])> = None;
        for (i, px) in row.iter_mut().enumerate() {
            // X = hc; the core's HBlank releases from hc = 17. On the console the 224 pixels
            // are tile columns 2 to 29 whole (the first fetch that shows is $B802)
            let x = if b.hw { 16 + from + i } else { 17 + from + i };
            if x >= 240 {
                *px = 0; // ... and starts again at hc = 240
                continue;
            }
            let tx = (x & 0xFF) / 8;
            let planes = match cached {
                Some((t, p)) if t == tx => p,
                _ => {
                    let tile = m[b.map_base().wrapping_add(((y / 8) * 32 + tx) as u16) as usize] as u16;
                    let base = b.pattern_addr(tile);
                    let rd = |off: u16| m[base.wrapping_add(line as u16 + off) as usize];
                    let p = [rd(8), rd(16), rd(24)];
                    cached = Some((tx, p));
                    p
                }
            };
            let bit = 7 - (x & 7);
            let on = |v: u8| (v >> bit) & 1 != 0;
            *px = (if on(planes[0]) { 0xFF0000 } else { 0 }) | (if on(planes[1]) { 0x00FF00 } else { 0 }) | (if on(planes[2]) { 0x0000FF } else { 0 });
        }
    }
}
