-- de0Test.vhd
--
-- entity	de0Test		-testbench for: de0Board
-- architecture	testbench
--------------------------------------------------------------------------------
library ieee;
use ieee.std_logic_1164.all;

entity de0Test is
generic(	periodC	: time		:= 20 ns);
end entity de0Test;

architecture testbench of de0Test is

-- quartus vhdl netlist writer: BUFFER instead of OUT!	------------------------
-- workaround:	1. change component declaration
--		2. edit: sim/xcelium/de0Board...vho
  component de0Board is
  port(	clk50		: in	std_logic;		-- 50 MHz external clock
	key		: in	std_logic_vector( 1 downto 0);	-- act. L
	led		: buffer std_logic_vector( 7 downto 0);	-- act. H
	dramCsN		: buffer std_logic;		-- L: chip select
	epcsCsN		: buffer std_logic;		-- L: chip sel.	CS#
	gSensorCs	: buffer std_logic;		-- H: chip sel. I2C-mode
	adcCsN		: buffer std_logic;		-- L: chip select
	gpio1		: buffer std_logic_vector(33 downto 0));
  end component de0Board;

  signal clk50		: std_logic;
  signal key		: std_logic_vector( 1 downto 0);
  signal led		: std_logic_vector( 7 downto 0);
  signal dramCsN	: std_logic;		-- L: chip select
  signal epcsCsN	: std_logic;		-- L: chip sel.	CS#
  signal gSensorCs	: std_logic;		-- H: chip sel. I2C-mode
  signal adcCsN		: std_logic;		-- L: chip select
  signal gpio1		: std_logic_vector(33 downto 0);
begin
  de0I: de0Board	port map (clk50, key, led,
			dramCsN, epcsCsN, gSensorCs, adcCsN, gpio1);

  clkP: process is
  begin
	clk50 <= '0', '1' after periodC/2;
	wait for periodC;
  end process clkP;

  keyP: process is
  begin
	key <= "01", "11" after 2*periodC, "10" after 2000*periodC;
	wait;
  end process keyP;
end architecture testbench;
