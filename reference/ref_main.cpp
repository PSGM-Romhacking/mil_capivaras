// Reference: the MiSTer core simulated with Verilator, without a window, to compare
// frames and RAM with Mil Capivaras. The options match the emulator's:
//   pv1000-ref GAME.bin|.zip --shot FRAMES out.ppm [--press-start] [--dump-ram ram.bin]
// --press-start: Start on frames 120-129, then Fire 1 + right every other 20 frames from 200.
#include "sim.h"
#include <string>

static std::vector<uint8_t> read_file(const std::string &p) {
	bool zip = p.size() > 4 && p.compare(p.size() - 4, 4, ".zip") == 0;
	std::string q = "'";
	for (char c : p) q += (c == '\'') ? std::string("'\\''") : std::string(1, c);
	q += "'";
	FILE *f = zip ? popen(("unzip -p " + q).c_str(), "r") : fopen(p.c_str(), "rb");
	if (!f) { perror(p.c_str()); exit(1); }
	std::vector<uint8_t> d; uint8_t buf[4096]; size_t n;
	while ((n = fread(buf, 1, sizeof buf, f)) > 0) d.insert(d.end(), buf, buf + n);
	if (zip) pclose(f); else fclose(f);
	return d;
}

int main(int argc, char **argv) {
	const char *rom = nullptr, *shot = nullptr, *dump = nullptr;
	int frames = 600; bool press = false;
	for (int i = 1; i < argc; i++) {
		if (!strcmp(argv[i], "--shot") && i + 2 < argc) { frames = atoi(argv[++i]); shot = argv[++i]; }
		else if (!strcmp(argv[i], "--dump-ram") && i + 1 < argc) dump = argv[++i];
		else if (!strcmp(argv[i], "--press-start")) press = true;
		else rom = argv[i];
	}
	if (!rom) { fprintf(stderr, "usage: %s GAME [--shot N out.ppm] [--press-start] [--dump-ram ram.bin]\n", argv[0]); return 1; }
	auto ctx = std::make_unique<VerilatedContext>();
	Sim sim(ctx.get());
	sim.load(read_file(rom));
	for (int f = 0; f < frames; f++) {
		if (press) sim.top->joy0 = (f >= 120 && f < 130) ? (1u << 7) : (f >= 200 && (f / 20) % 2) ? (1u << 4 | 1u << 0) : 0;
		sim.run_frame(); sim.samples.clear();
	}
	if (shot) {
		FILE *f = fopen(shot, "wb");
		fprintf(f, "P6\n224 224\n255\n");
		for (int y = 0; y < 224; y++) for (int x = 0; x < 224; x++) {
			uint32_t p = sim.fb[y * FBW + x];
			uint8_t rgb[3] = {(uint8_t)(p >> 16), (uint8_t)(p >> 8), (uint8_t)p};
			fwrite(rgb, 1, 3, f);
		}
		fclose(f);
	}
	if (dump) {
		FILE *f = fopen(dump, "wb");
		for (int i = 0; i < 65536; i++) { uint8_t b = sim.r->sim_top__DOT__mem[i]; fwrite(&b, 1, 1, f); }
		fclose(f);
	}
	return 0;
}
