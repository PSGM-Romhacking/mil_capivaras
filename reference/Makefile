# The MiSTer core of the PV-1000 simulated with Verilator (no window), to compare
# Mil Capivaras with the hardware described in Verilog. See README.md.
#   make -C reference        builds reference/pv1000-ref

RTL := $(addprefix rtl/,sim_top.v CasioPV1000.v cpu_z80.v \
       tv80/tv80s.v tv80/tv80_core.v tv80/tv80_alu.v tv80/tv80_mcode.v tv80/tv80_reg.v)

pv1000-ref: $(RTL) ref_main.cpp sim.h
	# TV80_REFRESH: the R register counts like on a real Z80 (the MiSTer core uses
	# T80, which does); without it, LD A,R always reads 0
	verilator --cc --exe --build -j $(shell nproc) --top-module sim_top +define+TV80_REFRESH \
	  --default-language 1800-2017 -O3 --x-assign fast --x-initial fast --noassert \
	  -Wno-fatal -Wno-lint -Wno-style -Wno-MULTIDRIVEN -Wno-COMBDLY \
	  -CFLAGS "-O3 -std=c++17 -Wno-error -I$(CURDIR)" \
	  --Mdir obj_dir -o pv1000-ref $(RTL) ref_main.cpp
	install -m 755 obj_dir/pv1000-ref ./pv1000-ref

clean:
	rm -rf obj_dir pv1000-ref

.PHONY: clean
