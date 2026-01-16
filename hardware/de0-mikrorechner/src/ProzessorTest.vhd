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
        clk : in std_logic; -- Takt-Signal für die gesamte Architektur
        rstN : in STD_LOGIC;
        iAddr : out STD_LOGIC_VECTOR(9 downto 0);
        iData : in STD_LOGIC_VECTOR(31 downto 0);
        dnWE : out STD_LOGIC;
        dAddr : out STD_LOGIC_VECTOR(9 downto 0);
        dDataI : in STD_LOGIC_VECTOR(31 downto 0);
        dDataO : out STD_LOGIC_VECTOR(31 downto 0)
    );
    end component Prozessor;

    signal clk, dnWE : STD_LOGIC := '0';
    signal rstN : STD_LOGIC := '1';
    signal iAddr, dAddr : STD_LOGIC_VECTOR (9 downto 0);
    signal iData, dDataI, dDataO : STD_LOGIC_VECTOR(31 downto 0);
begin

    ProzessorI : Prozessor port map (clk, rstN, iAddr, iData, dnWE, dAddr, dDataI, dDataO);

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