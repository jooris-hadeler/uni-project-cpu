library IEEE;
use IEEE.STD_LOGIC_1164.ALL;
use IEEE.NUMERIC_STD.ALL;
use work.memPkg.all;


entity ProzessorTest is
    generic(	
    periodC	: time		:= 20 ns);
end entity;

architecture testbench of ProzessorTest is

    component Prozessor is
        port (
            clk : in std_logic 
        );
    end component Prozessor;

    signal clk : STD_LOGIC := '0';
begin

    ProzessorI : Prozessor port map (clk);

    ProzessorPr: process is
    begin
        wait for periodC;
        for i in 0 to 100 loop
            clk <= '1'; 
            wait for periodC/2;
            clk <= '0';
            wait for periodC/2;
        end loop; 
        wait;
    end process ProzessorPr;
end architecture testbench;