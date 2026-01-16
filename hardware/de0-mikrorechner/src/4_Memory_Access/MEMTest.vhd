library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
library work;
use work.alu_opcode.all;

entity MEMTest is
    generic(	
        periodC	: time		:= 20 ns;
        cyclesC	: integer	:= 100);
    end entity MEMTest;

architecture testbench of MEMTest is
    component MEM is
    port (
        pc_in, address_in, write_data : in std_logic_vector(31 downto 0);
        clk, mem_write, mem_to_reg_in, reg_write_in, pc_src_in : in std_logic;
        write_reg: in std_logic_vector(4 downto 0);
        read_data_out, adress_out, pc_out: out std_logic_vector(31 downto 0);
        mem_to_reg_WB, reg_write_WB, pc_src_IF : out std_logic;
        write_reg_out: out std_logic_vector(4 downto 0);
        dnWE : out STD_LOGIC;
        dAddr : out STD_LOGIC_VECTOR(9 downto 0);
        dDataI : in STD_LOGIC_VECTOR(31 downto 0);
        dDataO : out STD_LOGIC_VECTOR(31 downto 0)
    );
    end component MEM;

    signal pc_in_in, adress_in_in, write_data_in: std_logic_vector(31 downto 0);
    signal clk_in, mem_write_in, mem_to_reg_in_in, reg_write_in_in, pc_src_in_in : std_logic;
    signal write_reg_in: std_logic_vector(4 downto 0);
    signal read_data_out_out, adress_out_out, pc_out_out: std_logic_vector(31 downto 0);
    signal mem_to_reg_WB_out, reg_write_WB_out, pc_src_IF_out, dnWE: std_logic;
    signal write_reg_out_out: std_logic_vector(4 downto 0);
    signal dAddr : STD_LOGIC_VECTOR(9 downto 0);
    signal dDataI, dDataO : STD_LOGIC_VECTOR(31 downto 0);
begin
    MEMI: MEM port map (pc_in_in, adress_in_in, write_data_in, clk_in, mem_write_in,
        mem_to_reg_in_in, reg_write_in_in, pc_src_in_in,
        write_reg_in, read_data_out_out, adress_out_out, pc_out_out, mem_to_reg_WB_out,
        reg_write_WB_out, pc_src_IF_out, write_reg_out_out, dnWE, dAddr, dDataI, dDataO);
    MEMP: process is
    begin
        pc_in_in <= STD_LOGIC_VECTOR(to_signed(1, 32));
        adress_in_in <= STD_LOGIC_VECTOR(to_signed(1, 32));

        write_data_in <= "00000000000000000000000000000001";

        mem_to_reg_in_in <= '1';
        reg_write_in_in <= '1';
        
        write_reg_in <= "00000";

        clk_in <= '0';
        wait for periodC;
        mem_write_in <= '0';
        clk_in <= '1'; 
        wait for periodC;
        clk_in <= '0';
        wait for periodC;
        clk_in <= '1'; 
        wait for periodC;
        mem_write_in <= '1';
        wait;
    end process MEMP;
end architecture testbench;		  
