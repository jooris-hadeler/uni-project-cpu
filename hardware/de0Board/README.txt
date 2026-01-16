README.txt	ajm						15-dec-2022
-de0Board.tgz	generic wrapper for DE0-Nano
--------------------------------------------------------------------------------

Makefile	-non graphical compilation and programming
de0Board.vhd	-sample top level wrapper, to be edited!
de0Board.qpf	-files for Quartus
de0Board.qsf	-
de0Board.sdc	-
quartus2.ini	-search path for external EDA programs, adapt to $tamsSW/...

doc/		-additional datasheets: DE0-board, PCB devices, etc.
src/		-... VHDL design sources
sim/hdl/	-sim. directory (xmsim) - pre-synthesis
sim/net/	-sim. directory (xmsim) - for Quartus netlist from xcelium
sim/xcelium/	-Quartus generated VHDL (Verilog) netlists for simulation

qProgram/	-saved programming files from prev. runs
qOutput/	-Quartus generated output: reports, programming (de0Board.sof)
		 deleted by Makefile

Where to start the tools?
--------------------------------------------------------------------------------
de0Board	  synthesis working directory:	quartus
de0Board/sim/hdl  pre-synthesis simulation:	xmvhdl / xmelab / xmsim
de0Board/sim/net  netlist simulation:		xmvlog / xmvhdl / xmelab / xmsim
--------------------------------------------------------------------------------
README.txt - end
