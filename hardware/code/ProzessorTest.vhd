library IEEE;
use IEEE.STD_LOGIC_1164.ALL;
use IEEE.NUMERIC_STD.ALL;
use work.memPkg.all;


entity ProzessorTest is
    generic(	
    periodC	: time		:= 10 ns);
end entity;

architecture testbench of ProzessorTest is

    component Prozessor is
        port (
            clk : in std_logic -- Takt-Signal fÃ¼r die gesamte Architektur
        );
    end component Prozessor;

    signal clk : STD_LOGIC;
begin

    ProzessorI : Prozessor port map (clk);

    ProzessorPr: process is
    begin
        clk <= '0';
        wait for periodC;
        clk <= '1'; 
        wait for periodC;
        wait;
    end process ProzessorPr;
end architecture testbench;