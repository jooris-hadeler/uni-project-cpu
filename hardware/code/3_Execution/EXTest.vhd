library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
library work;
use work.alu_opcode.all;

entity EXTest is
    generic(	
        periodC	: time		:= 10 ns;
        cyclesC	: integer	:= 100);
    end entity EXTest;

architecture testbench of EXTest is

    component EX is
    port (
        imm, pc, alu_val, reg_val: in std_logic_vector(31 downto 0);
        alu_op, rt, rd: in std_logic_vector(4 downto 0);
        clk, reg_dest, reg_write_EX, alu_src, pc_src, mem_write, mem_to_reg_EX, jr: in std_logic; -- mux_sel für alu, write_sel für befehls_mux unten bild            
        pc_out, out_result, data: out std_logic_vector(31 downto 0);
        write_reg: out std_logic_vector(4 downto 0);
        mem_write_out, mem_to_reg_MEM, reg_write_MEM, pc_src_MEM : out std_logic);
    end component EX;
    
    signal imm_in, pc_in, alu_val_in, reg_val_in: std_logic_vector(31 downto 0);
    signal alu_op_in, rt_in, rd_in:  std_logic_vector(4 downto 0);
    signal clk_in, reg_dest_in, reg_write_EX_in, alu_src_in, pc_src_in, mem_write_in, mem_to_reg_EX_in, jr_in: std_logic;
    signal pc_out_out, out_result_out, data_out: std_logic_vector(31 downto 0);
    signal write_reg_out: std_logic_vector(4 downto 0); 
    signal wE_out_out, mem_to_reg_MEM_out, reg_write_MEM_out, pc_src_IF_out : std_logic;

begin
    EXI: EX	port map (imm_in, pc_in, alu_val_in, reg_val_in, alu_op_in, rt_in, rd_in, clk_in, reg_dest_in, reg_write_EX_in, alu_src_in, pc_src_in, mem_write_in, mem_to_reg_EX_in, jr_in,
        pc_out_out, out_result_out, data_out, write_reg_out, wE_out_out, mem_to_reg_MEM_out, reg_write_MEM_out, pc_src_IF_out);

    EXPr: process is
    begin

        imm_in <= "00000000000000000000000000000001";
        pc_in <= "00000000000000000000000000000001";
        alu_val_in <= "00000000000000000000000000000001";
        reg_val_in <= "00000000000000000000000000000001";

        alu_op_in <= "00001";
        rt_in <= "00001";
        rd_in <= "00001";
        
        reg_dest_in <= '1';
        reg_write_EX_in <= '1';
        alu_src_in <= '1';
        pc_src_in <= '1';
        mem_write_in <= '1';
        mem_to_reg_EX_in <= '1';
        jr_in <= '1';

        clk_in <= '0';
        wait for periodC;
        clk_in <= '1'; 
        wait for periodC;
        wait;
    end process EXPr;
end architecture testbench;		  


