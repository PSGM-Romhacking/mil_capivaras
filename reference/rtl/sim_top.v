// Simulation top: replaces the MiSTer emu/sys (Casio_PV-1000.sv).
//
// To run in real time under Verilator, the 71.59 MHz clk_sys was removed:
//  - IO and interrupts run on the falling edge of clk_cpu (3.58 MHz);
//  - the sound runs on clk_vdp (5.97 MHz) with divider 170 (same frequency as 512 @ 17.9 MHz);
//  - the core's video state machine is idle; the C++ side draws from VRAM.
// The cartridge is loaded through the dl_* ports during reset, like the MiSTer ioctl.

module sim_top(
	input         clk_cpu,
	input         clk_vdp,
	input         reset,

	input         dl_we,
	input  [15:0] dl_addr,
	input   [7:0] dl_data,

	input  [31:0] joy0,
	input  [31:0] joy1,

	output        hblank,
	output        vblank,
	output        hsync,
	output        vsync,
	output  [7:0] audio
);

wire [15:0] ram_a;
wire        ram_we_n, ram_ce_n;
wire  [7:0] ram_do;
reg   [7:0] ram_di;

// 64 KB RAM; read/written on the CPU clock's falling edge (the address comes out on the rising edge)
reg [7:0] mem[0:65535] /*verilator public_flat_rd*/;

always @(negedge clk_cpu) begin
	if(dl_we) mem[dl_addr] <= dl_data;
	else if(~ram_we_n) mem[ram_a] <= ram_do;
	ram_di <= mem[ram_a];
end

CasioPV1000 #(.SND_DIV(11'd170)) console
(
	.clk(1'b0),
	.clk_cpu(clk_cpu),
	.clk_vdp(clk_vdp),
	.clk_snd(clk_vdp),
	.reset(reset),

	.audio(audio),

	.cpu_ram_a_o(ram_a),
	.cpu_ram_ce_n_o(ram_ce_n),
	.cpu_ram_we_n_o(ram_we_n),
	.cpu_ram_d_i(ram_di),
	.cpu_ram_d_o(ram_do),

	.joy0(joy0),
	.joy1(joy1),

	.HBlank(hblank),
	.HSync(hsync),
	.VBlank(vblank),
	.VSync(vsync),

	.vram_a(),
	.vram_di(8'h00),

	.video()
);

endmodule
