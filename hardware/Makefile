GHDL = ghdl
WORKDIR = work
SYNWORK = de0-mikrorechner/sim/ghdl/work

basics:
	$(GHDL) -a -fsynopsys --workdir=$(WORKDIR) "references/memorySim/memPkg.vhd"
	$(GHDL) -a -fsynopsys --workdir=$(WORKDIR) "references/memorySim/rom.vhd"
	$(GHDL) -a -fsynopsys --workdir=$(WORKDIR) "references/memorySim/ramB.vhd"
	$(GHDL) -a -fsynopsys --workdir=$(WORKDIR) "references/memorySim/ramIO.vhd"

	$(GHDL) -a --workdir=$(WORKDIR) "code/2_Instruction_Decode/funct_codes.vhd"
	$(GHDL) -a --workdir=$(WORKDIR) "code/2_Instruction_Decode/opcodes.vhd"
	$(GHDL) -a --workdir=$(WORKDIR) "code/2_Instruction_Decode/registerbank.vhd"
	$(GHDL) -a --workdir=$(WORKDIR) "code/2_Instruction_Decode/registerbankTest.vhd"
	$(GHDL) -e --workdir=$(WORKDIR) "registerbankTest"
	$(GHDL) -r --workdir=$(WORKDIR) "registerbankTest" --vcd=work/registerbankTest.vcd

	$(GHDL) -a --workdir=$(WORKDIR) "code/3_Execution/alu_opcode.vhd"
	$(GHDL) -a --workdir=$(WORKDIR) "code/3_Execution/alu.vhd"
	$(GHDL) -a --workdir=$(WORKDIR) "code/3_Execution/aluTest.vhd"
	$(GHDL) -e --workdir=$(WORKDIR) "aluTest"
	$(GHDL) -r --workdir=$(WORKDIR) "aluTest" --vcd=work/aluTest.vcd

ram:
	$(GHDL) -a -fsynopsys --workdir=$(WORKDIR) "code/ram_tb.vhd"
	$(GHDL) -e -fsynopsys --workdir=$(WORKDIR) "ram_tb"
	$(GHDL) -r -fsynopsys --workdir=$(WORKDIR) "ram_tb" --vcd=work/ram_tb.vcd

rom:
	$(GHDL) -a -fsynopsys --workdir=$(WORKDIR) "code/rom_tb.vhd"
	$(GHDL) -e -fsynopsys --workdir=$(WORKDIR) "rom_tb"
	$(GHDL) -r -fsynopsys --workdir=$(WORKDIR) "rom_tb" --vcd=work/rom_tb.vcd


IF:
	$(GHDL) -a -fsynopsys --workdir=$(WORKDIR) "code/1_Instruction_Fetch/instF.vhd"
	$(GHDL) -a -fsynopsys --workdir=$(WORKDIR) "code/1_Instruction_Fetch/instFTest.vhd"
	$(GHDL) -e -fsynopsys --workdir=$(WORKDIR) "instFTest"
	$(GHDL) -r -fsynopsys --workdir=$(WORKDIR) "instfTest" --vcd=work/instFTest.vcd

ID:
	$(GHDL) -a --workdir=$(WORKDIR) "code/2_Instruction_Decode/ID.vhd"
	$(GHDL) -a --workdir=$(WORKDIR) "code/2_Instruction_Decode/IDTest.vhd"
	$(GHDL) -e --workdir=$(WORKDIR) "IDTest"
	$(GHDL) -r --workdir=$(WORKDIR) "IDTest" --vcd=work/IDTest.vcd

EX:
	$(GHDL) -a --workdir=$(WORKDIR) "code/3_Execution/EX.vhd"
	$(GHDL) -a --workdir=$(WORKDIR) "code/3_Execution/EXTest.vhd"
	$(GHDL) -e --workdir=$(WORKDIR) "EXTest"
	$(GHDL) -r --workdir=$(WORKDIR) "EXTest" --vcd=work/EXTest.vcd
MEM:
	$(GHDL) -a -fsynopsys --workdir=$(WORKDIR) "code/4_Memory_Access/MEM.vhd"
	$(GHDL) -a -fsynopsys --workdir=$(WORKDIR) "code/4_Memory_Access/MEMTest.vhd"
	$(GHDL) -e -fsynopsys --workdir=$(WORKDIR) "MEMTest"
	$(GHDL) -r -fsynopsys --workdir=$(WORKDIR) "MEMTest" --vcd=work/MEMTest.vcd
WB:
	$(GHDL) -a --workdir=$(WORKDIR) "code/5_Write_Back/WB.vhd"
	$(GHDL) -a --workdir=$(WORKDIR) "code/5_Write_Back/WBTest.vhd"
	$(GHDL) -e --workdir=$(WORKDIR) "WBTest"
	$(GHDL) -r --workdir=$(WORKDIR) "WBTest" --vcd=work/WBFTest.vcd

processor:
	$(GHDL) -a -fsynopsys --workdir=$(WORKDIR) "code/Prozessor.vhd"
	$(GHDL) -a -fsynopsys --workdir=$(WORKDIR) "code/ProzessorTest.vhd"
	$(GHDL) -e -fsynopsys --workdir=$(WORKDIR) "ProzessorTest" 
	$(GHDL) -r -fsynopsys --workdir=$(WORKDIR) "ProzessorTest" --vcd=work/ProzessorTest.vcd

synthproc: 
	$(GHDL) -a -fsynopsys --workdir=$(SYNWORK) "de0-mikrorechner/src/memPkg.vhd"
	$(GHDL) -a -fsynopsys --workdir=$(SYNWORK) "de0-mikrorechner/src/rom.vhd"
	$(GHDL) -a -fsynopsys --workdir=$(SYNWORK) "de0-mikrorechner/src/ramB.vhd"
	$(GHDL) -a -fsynopsys --workdir=$(SYNWORK) "de0-mikrorechner/src/ramIO.vhd"

	$(GHDL) -a --workdir=$(SYNWORK) "de0-mikrorechner/src/2_Instruction_Decode/funct_codes.vhd"
	$(GHDL) -a --workdir=$(SYNWORK) "de0-mikrorechner/src/2_Instruction_Decode/opcodes.vhd"
	$(GHDL) -a --workdir=$(SYNWORK) "de0-mikrorechner/src/2_Instruction_Decode/registerbank.vhd"
	$(GHDL) -a --workdir=$(SYNWORK) "de0-mikrorechner/src/2_Instruction_Decode/registerbankTest.vhd"
	$(GHDL) -e --workdir=$(SYNWORK) "registerbankTest"
	$(GHDL) -r --workdir=$(SYNWORK) "registerbankTest" --vcd=de0-mikrorechner/sim/ghdl/work/registerbankTest.vcd

	$(GHDL) -a --workdir=$(SYNWORK) "de0-mikrorechner/src/3_Execution/alu_opcode.vhd"
	$(GHDL) -a --workdir=$(SYNWORK) "de0-mikrorechner/src/3_Execution/alu.vhd"
	$(GHDL) -a --workdir=$(SYNWORK) "de0-mikrorechner/src/3_Execution/aluTest.vhd"
	$(GHDL) -e --workdir=$(SYNWORK) "aluTest"
	$(GHDL) -r --workdir=$(SYNWORK) "aluTest" --vcd=de0-mikrorechner/sim/ghdl/work/aluTest.vcd

	$(GHDL) -a -fsynopsys --workdir=$(SYNWORK) "de0-mikrorechner/src/1_Instruction_Fetch/instF.vhd"
	$(GHDL) -a -fsynopsys --workdir=$(SYNWORK) "de0-mikrorechner/src/1_Instruction_Fetch/instFTest.vhd"
	$(GHDL) -e -fsynopsys --workdir=$(SYNWORK) "instFTest"
	$(GHDL) -r -fsynopsys --workdir=$(SYNWORK) "instfTest" --vcd=de0-mikrorechner/sim/ghdl/work/instFTest.vcd

	$(GHDL) -a --workdir=$(SYNWORK) "de0-mikrorechner/src/2_Instruction_Decode/ID.vhd"
	$(GHDL) -a --workdir=$(SYNWORK) "de0-mikrorechner/src/2_Instruction_Decode/IDTest.vhd"
	$(GHDL) -e --workdir=$(SYNWORK) "IDTest"
	$(GHDL) -r --workdir=$(SYNWORK) "IDTest" --vcd=de0-mikrorechner/sim/ghdl/work/IDTest.vcd

	$(GHDL) -a --workdir=$(SYNWORK) "de0-mikrorechner/src/3_Execution/EX.vhd"
	$(GHDL) -a --workdir=$(SYNWORK) "de0-mikrorechner/src/3_Execution/EXTest.vhd"
	$(GHDL) -e --workdir=$(SYNWORK) "EXTest"
	$(GHDL) -r --workdir=$(SYNWORK) "EXTest" --vcd=de0-mikrorechner/sim/ghdl/work/EXTest.vcd

	$(GHDL) -a -fsynopsys --workdir=$(SYNWORK) "de0-mikrorechner/src/4_Memory_Access/MEM.vhd"
	$(GHDL) -a -fsynopsys --workdir=$(SYNWORK) "de0-mikrorechner/src/4_Memory_Access/MEMTest.vhd"
	$(GHDL) -e -fsynopsys --workdir=$(SYNWORK) "MEMTest"
	$(GHDL) -r -fsynopsys --workdir=$(SYNWORK) "MEMTest" --vcd=de0-mikrorechner/sim/ghdl/work/MEMTest.vcd

	$(GHDL) -a --workdir=$(SYNWORK) "de0-mikrorechner/src/5_Write_Back/WB.vhd"
	$(GHDL) -a --workdir=$(SYNWORK) "de0-mikrorechner/src/5_Write_Back/WBTest.vhd"
	$(GHDL) -e --workdir=$(SYNWORK) "WBTest"
	$(GHDL) -r --workdir=$(SYNWORK) "WBTest" --vcd=de0-mikrorechner/sim/ghdl/work/WBFTest.vcd

	$(GHDL) -a -fsynopsys --workdir=$(SYNWORK) "de0-mikrorechner/src/Prozessor.vhd"
	$(GHDL) -a -fsynopsys --workdir=$(SYNWORK) "de0-mikrorechner/src/procTest.vhd"
	$(GHDL) -e -fsynopsys --workdir=$(SYNWORK) "procTest" 
	$(GHDL) -r -fsynopsys --workdir=$(SYNWORK) "procTest" --vcd=de0-mikrorechner/sim/ghdl/work/procTest.vcd

synthde0: 
	$(GHDL) -a -fsynopsys --workdir=$(SYNWORK) "de0-mikrorechner/src/memPkg.vhd"
	$(GHDL) -a -fsynopsys --workdir=$(SYNWORK) "de0-mikrorechner/src/rom.vhd"
	$(GHDL) -a -fsynopsys --workdir=$(SYNWORK) "de0-mikrorechner/src/ramB.vhd"
	$(GHDL) -a -fsynopsys --workdir=$(SYNWORK) "de0-mikrorechner/src/ramIO.vhd"

	$(GHDL) -a --workdir=$(SYNWORK) "de0-mikrorechner/src/2_Instruction_Decode/funct_codes.vhd"
	$(GHDL) -a --workdir=$(SYNWORK) "de0-mikrorechner/src/2_Instruction_Decode/opcodes.vhd"
	$(GHDL) -a --workdir=$(SYNWORK) "de0-mikrorechner/src/2_Instruction_Decode/registerbank.vhd"
	$(GHDL) -a --workdir=$(SYNWORK) "de0-mikrorechner/src/2_Instruction_Decode/registerbankTest.vhd"
	$(GHDL) -e --workdir=$(SYNWORK) "registerbankTest"
	$(GHDL) -r --workdir=$(SYNWORK) "registerbankTest" --vcd=de0-mikrorechner/sim/ghdl/work/registerbankTest.vcd

	$(GHDL) -a --workdir=$(SYNWORK) "de0-mikrorechner/src/3_Execution/alu_opcode.vhd"
	$(GHDL) -a --workdir=$(SYNWORK) "de0-mikrorechner/src/3_Execution/alu.vhd"
	$(GHDL) -a --workdir=$(SYNWORK) "de0-mikrorechner/src/3_Execution/aluTest.vhd"
	$(GHDL) -e --workdir=$(SYNWORK) "aluTest"
	$(GHDL) -r --workdir=$(SYNWORK) "aluTest" --vcd=de0-mikrorechner/sim/ghdl/work/aluTest.vcd

	$(GHDL) -a -fsynopsys --workdir=$(SYNWORK) "de0-mikrorechner/src/1_Instruction_Fetch/instF.vhd"
	$(GHDL) -a -fsynopsys --workdir=$(SYNWORK) "de0-mikrorechner/src/1_Instruction_Fetch/instFTest.vhd"
	$(GHDL) -e -fsynopsys --workdir=$(SYNWORK) "instFTest"
	$(GHDL) -r -fsynopsys --workdir=$(SYNWORK) "instfTest" --vcd=de0-mikrorechner/sim/ghdl/work/instFTest.vcd

	$(GHDL) -a --workdir=$(SYNWORK) "de0-mikrorechner/src/2_Instruction_Decode/ID.vhd"
	$(GHDL) -a --workdir=$(SYNWORK) "de0-mikrorechner/src/2_Instruction_Decode/IDTest.vhd"
	$(GHDL) -e --workdir=$(SYNWORK) "IDTest"
	$(GHDL) -r --workdir=$(SYNWORK) "IDTest" --vcd=de0-mikrorechner/sim/ghdl/work/IDTest.vcd

	$(GHDL) -a --workdir=$(SYNWORK) "de0-mikrorechner/src/3_Execution/EX.vhd"
	$(GHDL) -a --workdir=$(SYNWORK) "de0-mikrorechner/src/3_Execution/EXTest.vhd"
	$(GHDL) -e --workdir=$(SYNWORK) "EXTest"
	$(GHDL) -r --workdir=$(SYNWORK) "EXTest" --vcd=de0-mikrorechner/sim/ghdl/work/EXTest.vcd

	$(GHDL) -a -fsynopsys --workdir=$(SYNWORK) "de0-mikrorechner/src/4_Memory_Access/MEM.vhd"
	$(GHDL) -a -fsynopsys --workdir=$(SYNWORK) "de0-mikrorechner/src/4_Memory_Access/MEMTest.vhd"
	$(GHDL) -e -fsynopsys --workdir=$(SYNWORK) "MEMTest"
	$(GHDL) -r -fsynopsys --workdir=$(SYNWORK) "MEMTest" --vcd=de0-mikrorechner/sim/ghdl/work/MEMTest.vcd

	$(GHDL) -a --workdir=$(SYNWORK) "de0-mikrorechner/src/5_Write_Back/WB.vhd"
	$(GHDL) -a --workdir=$(SYNWORK) "de0-mikrorechner/src/5_Write_Back/WBTest.vhd"
	$(GHDL) -e --workdir=$(SYNWORK) "WBTest"
	$(GHDL) -r --workdir=$(SYNWORK) "WBTest" --vcd=de0-mikrorechner/sim/ghdl/work/WBFTest.vcd

	$(GHDL) -a -fsynopsys --workdir=$(SYNWORK) "de0-mikrorechner/src/Prozessor.vhd"
	$(GHDL) -a -fsynopsys --workdir=$(SYNWORK) "de0-mikrorechner/cDisplay/cDispPkg.vhd"
	$(GHDL) -a -fsynopsys --workdir=$(SYNWORK) "de0-mikrorechner/src/procPkg.vhd"
	$(GHDL) -a -fsynopsys --workdir=$(SYNWORK) "de0-mikrorechner/src/de0Board.vhd"
	$(GHDL) -a -fsynopsys --workdir=$(SYNWORK) "de0-mikrorechner/src/de0Test.vhd"
	$(GHDL) -e -fsynopsys --workdir=$(SYNWORK) "de0Test" 
	$(GHDL) -r -fsynopsys --workdir=$(SYNWORK) "de0Test" --vcd=de0-mikrorechner/sim/ghdl/work/de0Test.vcd

