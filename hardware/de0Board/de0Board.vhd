-- de0Board.vhd
--------------------------------------------------------------------------------
--		ajm		29-dec-2014
--				-derived from: Terasic System Builder
--------------------------------------------------------------------------------
--
-- entity	de0Board	-generic wrapper for Terasic DE0-Nano
--				 prototyping board
-- architecture	wrapper
--
-- usage:	1. I/O setup	in entity
--				-comment out all unused ports!!!
--		2. declare	in architecture
--				-components to be used, see <myComponent>
--				-local signals
--		3. statements	in architecture
--				-component instances
--				-processes
--		4. Quartus	in file: de0Board.qsf
--				-add VHDL source files
--			or	GUI-setup within quartus
--------------------------------------------------------------------------------
library ieee;
use ieee.std_logic_1164.all;

-- entity	----------------------------------------------------------------
--------------------------------------------------------------------------------
entity de0Board is
port (	clk50		: in	std_logic;		-- 50 MHz external clock

	-- KEY		active LOW	----------------------------------------
	key		: in	std_logic_vector( 1 downto 0);

	-- DIP switch	0-Up / 1-Down	----------------------------------------
	switch		: in	std_logic_vector( 3 downto 0);

	-- LED		active HIGH	----------------------------------------
	led		: out	std_logic_vector( 7 downto 0);

	-- SDRAM 16Mx16	--------------------------------------------------------
	--		IS42S16160B 4M x 16 x 4 banks
	--		dram-IS42S16160B		=> page 8ff
	dramCsN		: out	std_logic;		-- L: chip select
	dramCke		: out	std_logic;		-- H: clock enable
	dramClk		: out	std_logic;		-- R: input-regs
	dramRasN	: out	std_logic;		-- L: row-addr. strobe
	dramCasN	: out	std_logic;		-- L: col-addr. strobe
	dramWeN		: out	std_logic;		-- L: write enable
	dramBa		: out	unsigned( 1 downto 0);	-- bank addr.
	dramAddr	: out	unsigned(12 downto 0);	-- address
	dramDqm		: out	unsigned( 1 downto 0);	-- byte dat.mask
	dramDq		: inout	std_logic_vector(15 downto 0);	-- data

--	-- EPCS		--------------------------------------------------------
--	--		Spansion S25FL064P: FPGA config. memory; 64M bit Flash
--	--		DE0-UserManual + epcs-S25FL064P + Altera Manuals
--	epcsCsN		: out	std_logic;		-- L: chip sel.	CS#
--	epcsDClk	: out	std_logic;		-- clock	SCK
--	epcsAsd		: out	std_logic;		-- ser.data out	SI/IO0
--	epcsData	: in	std_logic;		-- ser.data in	SO/IO1

	-- I2C EEPROM	--------------------------------------------------------
	--		Microchip 24LC02B 2K bit
	--		eeprom-24xx02			=> page 5ff
	i2cSClk		: out	std_logic;		-- SClock (bus master)
	i2cSDat		: inout	std_logic;		-- SData

	-- I2C Accelerometer	------------------------------------------------
	--		Analog Devices ADXL345
	--		accel-ADXL345			=> page 17ff
--	i2cSClk		: out	std_logic;		-- SClock (bus master)
--	i2cSDat		: inout	std_logic;		-- SData
	gSensorCs	: out	std_logic;		-- H: chip sel. I2C-mode
	gSensorInt	: in	std_logic;		-- interrupt	INT1

	-- AD converter	--------------------------------------------------------
	--		National Semiconductor ADC128S022
	--		adc-ADC128S022			=> page 2+7+16
	adcCsN		: out	std_logic;		-- L: chip select
	adcSClk		: out	std_logic;		-- clock [0,8-3,2MHz]
	adcSAddr	: out	std_logic;		-- command	DIN
	adcSData	: in	std_logic;		-- data		DOUT

	-- GPIO-0	--------------------------------------------------------
	--	top	DE0-UserManual			=> page 18
	gpio0		: inout	std_logic_vector(33 downto 0);
	gpio0In		: in	std_logic_vector( 1 downto 0);

	-- GPIO-1	--------------------------------------------------------
	--	bot.	DE0-UserManual			=> page 18
	gpio1		: inout	std_logic_vector(33 downto 0);
	gpio1In		: in	std_logic_vector( 1 downto 0);

	-- 2x13 GPIO	--------------------------------------------------------
	--	right	DE0-UserManual			=> page 21
	gpio2		: inout	std_logic_vector(12 downto 0);
	gpio2In		: in	std_logic_vector( 2 downto 0));
end entity de0Board;


-- architecture	----------------------------------------------------------------
--------------------------------------------------------------------------------
architecture wrapper of de0Board is
  ------------------------------------------------------------------------------
  -- component <myComponent> is
  --   generic	( ... );
  --   port	( ... );
  -- end component <myComponent>;
  ------------------------------------------------------------------------------
  -- signal	...

begin
  -- component instantitions
  ---------------------------------------------------------------------------
  -- topI: <myComponent>
  --	    generic map	(...)
  --	    port map	(...);

  -- processes
  ---------------------------------------------------------------------------
  -- topP: process (...) is
  --	begin
  --	  ...
  --	end process topP;

end architecture wrapper;

--------------------------------------------------------------------------------
-- de0Board.vhd - end
