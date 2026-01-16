README.txt	ajm						15-dec-2022
-de0-tlc.tgz	sample: Traffic Light Controller
--------------------------------------------------------------------------------

Makefile	-non graphical compilation and programming
de0Board.vhd	-sample top level wrapper, edit for clock divider!
de0Board.qpf	-files for Quartus
de0Board.qsf	-
de0Board.sdc	-
quartus2.ini	-search path for external EDA programs, adapt to $tamsSW/...

src/		-... VHDL design sources
src/tlcWalk.vhd -sample FSM
src/tlcTest.vhd	-testenv for:	tlcWalk
src/de0Test.vhd	-testenv for:	de0Board

sim/ghdl/	-ghdl simulation (hdl + net)
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
