README.txt	ajm						15-dec-2022
-de0serIO.tgz	sample code: serTX + ser RX
--------------------------------------------------------------------------------

Pinout + connections	--------------------------------------------------------
Funct.	VHDL		FPGA-Pin	Arduino-Pin	Function
Gnd	--		GPIO1 12		GND	Gnd
Tx	tx [gpio1(8)]	GPIO1 13	19	RX1	Rx
Rx	rx [gpio1(9)]	GPIO1 14	18	TX1	Tx

FPGA-Pin see: doc/DE0-UserManual.pdf, page 18: Fig.3-8 + 3-9

--------------------------------------------------------------------------------
Makefile	-non graphical compilation and programming
de0Board.vhd	-sample top level wrapper: echo function (Rx -> Tx)
de0Board.qpf	-files for Quartus
de0Board.qsf	-
de0Board.sdc	-
quartus2.ini	-search path for external EDA programs, adapt to $tamsSW/...

arduino/	-use Arduino as sender
fifo/		-internal buffer: fifo64

src/		-... VHDL design sources
src/serRx.vhd	-serial receiver
src/serTx.vhd	-serial transmitter

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
