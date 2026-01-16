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
use ieee.numeric_std.all;

-- entity	----------------------------------------------------------------
--------------------------------------------------------------------------------
entity de0Board is
port (	clk50		: in	std_logic;		-- 50 MHz external clock

	-- KEY		active LOW	----------------------------------------
	key		: in	std_logic_vector( 1 downto 0);

	-- DIP switch	0-Up / 1-Down	----------------------------------------
--	switch		: in	std_logic_vector( 3 downto 0);

	-- LED		active HIGH	----------------------------------------
	led		: out	std_logic_vector( 7 downto 0);

	-- SDRAM 16Mx16	--------------------------------------------------------
	--		IS42S16160B 4M x 16 x 4 banks
	--		dram-IS42S16160B		=> page 8ff
	dramCsN		: out	std_logic;		-- L: chip select
--	dramCke		: out	std_logic;		-- H: clock enable
--	dramClk		: out	std_logic;		-- R: input-regs
--	dramRasN	: out	std_logic;		-- L: row-addr. strobe
--	dramCasN	: out	std_logic;		-- L: col-addr. strobe
--	dramWeN		: out	std_logic;		-- L: write enable
--	dramBa		: out	unsigned( 1 downto 0);	-- bank addr.
--	dramAddr	: out	unsigned(12 downto 0);	-- address
--	dramDqm		: out	unsigned( 1 downto 0);	-- byte dat.mask
--	dramDq		: inout	std_logic_vector(15 downto 0);	-- data

--	-- EPCS		--------------------------------------------------------
--	--		Spansion S25FL064P: FPGA config. memory; 64M bit Flash
--	--		DE0-UserManual + epcs-S25FL064P + Altera Manuals
	epcsCsN		: out	std_logic;		-- L: chip sel.	CS#
--	epcsDClk	: out	std_logic;		-- clock	SCK
--	epcsAsd		: out	std_logic;		-- ser.data out	SI/IO0
--	epcsData	: in	std_logic;		-- ser.data in	SO/IO1

	-- I2C EEPROM	--------------------------------------------------------
	--		Microchip 24LC02B 2K bit
	--		eeprom-24xx02			=> page 5ff
--	i2cSClk		: out	std_logic;		-- SClock (bus master)
--	i2cSDat		: inout	std_logic;		-- SData

	-- I2C Accelerometer	------------------------------------------------
	--		Analog Devices ADXL345
	--		accel-ADXL345			=> page 17ff
--	i2cSClk		: out	std_logic;		-- SClock (bus master)
--	i2cSDat		: inout	std_logic;		-- SData
	gSensorCs	: out	std_logic;		-- H: chip sel. I2C-mode
--	gSensorInt	: in	std_logic;		-- interrupt	INT1

	-- AD converter	--------------------------------------------------------
	--		National Semiconductor ADC128S022
	--		adc-ADC128S022			=> page 2+7+16
	adcCsN		: out	std_logic;		-- L: chip select
--	adcSClk		: out	std_logic;		-- clock [0,8-3,2MHz]
--	adcSAddr	: out	std_logic;		-- command	DIN
--	adcSData	: in	std_logic;		-- data		DOUT

	-- GPIO-0	--------------------------------------------------------
	--	top	DE0-UserManual			=> page 18
--	gpio0		: inout	std_logic_vector(33 downto 0);
--	gpio0In		: in	std_logic_vector( 1 downto 0);

	-- GPIO-1	--------------------------------------------------------
	--	bot.	DE0-UserManual			=> page 18
	gpio1		: out	std_logic_vector(33 downto 0));
--	gpio1In		: in	std_logic_vector( 1 downto 0);

	-- 2x13 GPIO	--------------------------------------------------------
	--	right	DE0-UserManual			=> page 21
--	gpio2		: inout	std_logic_vector(12 downto 0);
--	gpio2In		: in	std_logic_vector( 2 downto 0));
end entity de0Board;


-- architecture	----------------------------------------------------------------
--------------------------------------------------------------------------------
architecture wrapper of de0Board is

  component tlcWalk is
  generic (	maxWalkC	: integer range 10 to 2000	:= 1000; 
		maxCarC		: integer range 10 to 2000	:=  800); 
  port    (	clk, rst	: in  std_logic;
		reqWalk		: in  std_logic;
		lightCar	: out std_logic_vector(1 to 3);
		lightWalk	: out std_logic_vector(1 to 2));
  end component tlcWalk;

  signal clkCnt	: unsigned(31 downto 0);
  signal liCar	: std_logic_vector(1 to 3);
  signal liWalk	: std_logic_vector(1 to 2);
begin
  -- disable unused hardware
  ---------------------------------------------------------------------------
  dramCsN	<= '1';
  epcsCsN	<= '1';
  gSensorCs	<= '0';
  adcCsN	<= '1';

  -- component instantitions
  ---------------------------------------------------------------------------
  tlcI: tlcWalk
  generic map (	maxWalkC	=> 15,
		maxCarC		=> 10) 
  port map  (	clk		=> clkCnt(25),	-- 25 synthesis / 3 simulation
--  port map  (	clk		=> clkCnt(3),	-- 25 synthesis / 3 simulation
		rst		=> key(1),
		reqWalk		=> not key(0),
		lightCar	=> liCar,
		lightWalk	=> liWalk);
  led <= liWalk(2) & liWalk(1) & "000" & liCar(3) & liCar(2) & liCar(1);

  -- gpio1 outer pins [2,4,6,8,10,12] == liCar & liWalk & gnd
  gpio1(0) <= liCar(1);
  gpio1(1) <= liCar(2);
  gpio1(3) <= liCar(3);
  gpio1(5) <= liWalk(1);
  gpio1(7) <= liWalk(2);

  clkP: process (clk50, key(1)) is
  begin
    if key(1) = '0' then	clkCnt <= (others => '0');
    elsif rising_edge(clk50) then
	if clkCnt = x"FFFFFFFF"  then
				clkCnt <= (others => '0');
	else			clkCnt <= clkCnt+1;
	end if;
    end if;
  end process clkP;
end architecture wrapper;
--------------------------------------------------------------------------------
-- de0Board.vhd - end
