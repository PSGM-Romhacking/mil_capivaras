// The MiSTer core simulated with Verilator: clocks, video capture and frame loop.
#pragma once
#include "Vsim_top.h"
#include "Vsim_top___024root.h"
#include "verilated.h"

#include <algorithm>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <memory>
#include <vector>

static constexpr double SYS_HZ   = 71590909.0;       // MiSTer clk_sys
static constexpr double BASE_HZ  = SYS_HZ / 2.0;     // simulation step: cpu = /10, vdp = /6
static constexpr double SND_HZ   = SYS_HZ / 12.0;    // the sound runs on clk_vdp
static constexpr int    AUDIO_HZ = 44100;
static constexpr int    FBW = 256, FBH = 256;

struct Sim {
	Vsim_top *top;
	Vsim_top___024root *r;
	int c_cpu = 0, c_vdp = 0;
	uint64_t cycles = 0;

	// video
	std::vector<uint32_t> fb = std::vector<uint32_t>(FBW * FBH, 0);
	int x = 0, y = 0, frame_w = 0, frame_h = 0;
	bool prev_hs = false, prev_vs = false, frame_ready = false;

	// audio (average of each sample interval)
	std::vector<int16_t> samples;
	uint32_t acc = 0, acc_n = 0;
	double frac = 0;

	explicit Sim(VerilatedContext *ctx) : top(new Vsim_top(ctx)), r(top->rootp) {
		top->clk_cpu = top->clk_vdp = 0;
		top->reset = 1;
		top->dl_we = 0;
		top->joy0 = top->joy1 = 0;
		top->eval();
	}
	~Sim() { top->final(); delete top; }

	// Current pixel, decoded like the core's video state machine: tile map at 0xB800,
	// patterns in ROM (or the PCG at 0xBC00 for tiles >= 0xE0), each tile row = 3 bytes
	// (R, G, B) at offsets +8, +16, +24.
	inline uint32_t pixel() {
		int X = r->sim_top__DOT__console__DOT__hc;
		int Y = (int)r->sim_top__DOT__console__DOT__vc - 16;
		if (Y < 0 || Y >= 192) {
			int bc = r->sim_top__DOT__console__DOT__border_color;
			return (bc & 1 ? 0xFF0000u : 0) | (bc & 2 ? 0x00FF00u : 0) | (bc & 4 ? 0x0000FFu : 0);
		}
		auto &m = r->sim_top__DOT__mem;
		uint8_t tile = m[0xB800 + (Y / 8) * 32 + (X & 0xFF) / 8];
		uint16_t base;
		if (tile < 0xE0 || r->sim_top__DOT__console__DOT__force_pattern)
			base = (uint16_t)((tile + (r->sim_top__DOT__console__DOT__pcg_bank ? 0x100 : 0)) * 32);
		else
			base = (uint16_t)(0xBC00 + (tile - 0xE0) * 32);
		int row = Y & 7, bit = 7 - (X & 7);
		uint32_t px = 0;
		if ((m[(uint16_t)(base + row + 8)]  >> bit) & 1) px |= 0xFF0000;
		if ((m[(uint16_t)(base + row + 16)] >> bit) & 1) px |= 0x00FF00;
		if ((m[(uint16_t)(base + row + 24)] >> bit) & 1) px |= 0x0000FF;
		return px;
	}

	void video_sample() {
		bool hs = top->hsync, vs = top->vsync;
		if (vs && !prev_vs) {
			frame_h = y; y = 0; x = 0; frame_ready = true;
		} else if (hs && !prev_hs) {
			if (x > 0) { frame_w = std::max(frame_w, x); y++; }
			x = 0;
		}
		prev_hs = hs; prev_vs = vs;
		if (!top->hblank && !top->vblank && x < FBW && y < FBH)
			fb[y * FBW + x++] = pixel();
	}

	void audio_sample() {
		acc += top->audio; acc_n++;
		frac += AUDIO_HZ;
		if (frac >= SND_HZ) {
			frac -= SND_HZ;
			int v = acc_n ? (int)(acc / acc_n) : 0;   // 0..192
			samples.push_back((int16_t)((v - 96) * 96));
			acc = acc_n = 0;
		}
	}

	// One 1/35.8 MHz step; eval() only runs when a clock changes.
	inline void tick() {
		bool ev = false, vdp_rise = false;
		if (++c_cpu == 5) { c_cpu = 0; top->clk_cpu ^= 1; ev = true; }
		if (++c_vdp == 3) { c_vdp = 0; top->clk_vdp ^= 1; vdp_rise = top->clk_vdp; ev = true; }
		if (ev) top->eval();
		if (vdp_rise) { video_sample(); audio_sample(); }
		cycles++;
	}

	void load(const std::vector<uint8_t> &rom) {
		top->reset = 1;
		for (size_t i = 0; i < rom.size() && i < 0x10000; i++) {
			top->dl_we = 1; top->dl_addr = (uint16_t)i; top->dl_data = rom[i];
			for (int k = 0; k < 10; k++) tick();   // one clk_cpu period per byte
		}
		top->dl_we = 0;
		reset();
	}

	void reset() {
		top->reset = 1;
		for (int i = 0; i < 2000; i++) tick();
		// release the reset always at the same phase of clk_cpu against clk_vdp (it
		// used to depend on the ROM size, through the 10 ticks per byte of load());
		// phase 10 is the one the Rust model reproduces (both clocks starting at 0)
		while (cycles % 30 != 10) tick();
		top->reset = 0;
	}

	// Runs one frame = lines 0..262, ending when the scan goes back to line 0 (as the
	// Rust emulator does; ending at VSync, line 259, made the inputs and the RAM dumps
	// fall ~3 lines apart). Capped so it can't hang if the video stops.
	void run_frame() {
		frame_ready = false;
		int prev_hc = r->sim_top__DOT__console__DOT__hc;
		for (int i = 0; i < 2000000; i++) {
			tick();
			int hc = r->sim_top__DOT__console__DOT__hc, vc = r->sim_top__DOT__console__DOT__vc;
			bool wrapped = hc == 0 && prev_hc != 0 && vc == 0;
			prev_hc = hc;
			if (wrapped) break;
		}
	}
};
