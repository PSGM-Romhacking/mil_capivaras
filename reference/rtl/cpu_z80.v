// Z80 wrapper for simulation: the same interface as the MiSTer core's rtl/cpu_z80.v,
// but using TV80 (Verilog) instead of T80 (VHDL), which Verilator cannot read.

module cpu_z80(
	input CLK_4M,
	input nRESET,
	input [7:0] SDD_IN,
	output [7:0] SDD_OUT,
	output [15:0] SDA,
	output nIORQ,
	output nMREQ,
	output nRD, nWR,
	input nINT, nNMI, nWAIT
);

	wire RFSH_n, MREQ_n;
	assign nMREQ = MREQ_n | ~RFSH_n;

	tv80s #(.Mode(0), .T2Write(1), .IOWait(1)) cpu(
		.reset_n(nRESET),
		.clk(CLK_4M),
		.wait_n(nWAIT),
		.int_n(nINT),
		.nmi_n(nNMI),
		.busrq_n(1'b1),
		.m1_n(),
		.mreq_n(MREQ_n),
		.iorq_n(nIORQ),
		.rd_n(nRD),
		.wr_n(nWR),
		.rfsh_n(RFSH_n),
		.halt_n(),
		.busak_n(),
		.A(SDA),
		.di(SDD_IN),
		.dout(SDD_OUT)
	);

endmodule
