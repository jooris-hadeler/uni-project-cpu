README.txt	ajm						15-dec-2022
-de0cDisplay.tgz	sample code: character display
--------------------------------------------------------------------------------

Makefile	-non graphical compilation and programming
de0Board.vhd	-sample design on how to use: cDisp14x6
de0Board.qpf	-files for Quartus
de0Board.qsf	-
de0Board.sdc	-
quartus2.ini	-search path for external EDA programs, adapt to $tamsSW/...

cDisplay/cDisp14x6.vhd	-display controller FSM
cDisplay/cDispPkg.vhd	-component: cDisp14x6, pllClk	type: cmdTy
		 => for external use
		 components and types used in cDisp14x6

cDisplay/pll/	-pll to generate 2MHz clocks from 50MHz input on DE0-board
		 must be used within de0Board.vhd, see example
cDisplay/rom/	-character ROM for cDisp14x6
		 internally used in cDisp14x6

doc/		-additional datasheets: DE0-board, PCD8544, etc.
src/		-... VHDL design sources

sim/hdl/	-sim. directory (xmsim) - pre-synthesis
sim/net/	-sim. directory (xmsim) - for Quartus netlist from xcelium
sim/xcelium/	-Quartus generated VHDL (Verilog) netlists for simulation

qProgram/	-saved programming files from prev. runs
qOutput		-Quartus generated output: reports, programming (de0Board.sof)
		 deleted by Makefile

Where to start the tools?
--------------------------------------------------------------------------------
de0Board	  synthesis working directory:	quartus
de0Board/sim/hdl  pre-synthesis simulation:	xmvhdl / xmelab / xmsim
de0Board/sim/net  netlist simulation:		xmvlog / xmvhdl / xmelab / xmsim
--------------------------------------------------------------------------------
README.txt - end
