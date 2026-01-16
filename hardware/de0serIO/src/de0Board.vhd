-- de0Board.vhd
--------------------------------------------------------------------------------
--		ajm		12-jun-2018
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
generic(baudG		: positive	:= 115200;  --19200;	-- Baud rate
	msEchoG		: positive	:= 1);		-- ms echo delay
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
--	gpio1		: inout	std_logic_vector(33 downto 0);
--	gpio1In		: in	std_logic_vector( 1 downto 0);

	-- 2x13 GPIO	--------------------------------------------------------
	--	right	DE0-UserManual			=> page 21
--	gpio2		: inout	std_logic_vector(12 downto 0);
--	gpio2In		: in	std_logic_vector( 2 downto 0));

	rx		: in	std_logic;	-- crossed Tx	= gpio1(9)
	tx		: out	std_logic);	-- crossed Rx	= gpio1(8)
end entity de0Board;


-- architecture	----------------------------------------------------------------
--------------------------------------------------------------------------------
architecture wrapper of de0Board is

  component fifo64 is
  port(	aclr		: in  std_logic;
	clock		: in  std_logic ;
	data		: in  std_logic_vector(7 downto 0);
	rdreq		: in  std_logic ;
	wrreq		: in  std_logic ;
	empty		: out std_logic ;
	q		: out std_logic_vector(7 downto 0));
  end component fifo64;

  component serRx is
  generic(
	clkFreq		: positive	:= 50000000;	-- clock Frequency
	baudRate	: positive	:=    57600);	-- baud rate
  port(	clk		: in  std_logic;		-- clock	[R]
	rstN		: in  std_logic;		-- reset	[L]
	rx		: in  std_logic;		-- rx line
	byte		: out std_logic_vector(7 downto 0);	-- data
	byteEn		: out std_logic);		-- data enable	[H]
  end component serRx;


  component serTx is
  generic(
	clkFreq		: positive	:= 50000000;	-- clock Frequency
	baudRate	: positive	:=    57600);	-- baud rate
  port(	clk		: in  std_logic;		-- clock	[R]
	rstN		: in  std_logic;		-- reset	[L]
	byte		: in  std_logic_vector(7 downto 0);	-- data
	byteReq		: in  std_logic;		-- data request	[R+F]
	byteAck		: out std_logic;		-- data acknow.	[R+F]
	tx		: out std_logic);		-- tx line
  end component serTx;

  constant ascii0	: std_logic_vector(7 downto 0)	:= x"30";
  constant ascii1	: std_logic_vector(7 downto 0)	:= x"31";
  constant ascii2	: std_logic_vector(7 downto 0)	:= x"32";
  constant ascii3	: std_logic_vector(7 downto 0)	:= x"33";
  constant ascii4	: std_logic_vector(7 downto 0)	:= x"34";
  constant ascii5	: std_logic_vector(7 downto 0)	:= x"35";
  constant ascii6	: std_logic_vector(7 downto 0)	:= x"36";
  constant ascii7	: std_logic_vector(7 downto 0)	:= x"37";
  constant ascii8	: std_logic_vector(7 downto 0)	:= x"38";
  constant ascii9	: std_logic_vector(7 downto 0)	:= x"39";
  constant asciia	: std_logic_vector(7 downto 0)	:= x"61";
  constant asciib	: std_logic_vector(7 downto 0)	:= x"62";
  constant asciic	: std_logic_vector(7 downto 0)	:= x"63";
  constant asciid	: std_logic_vector(7 downto 0)	:= x"64";
  constant asciie	: std_logic_vector(7 downto 0)	:= x"65";
  constant asciif	: std_logic_vector(7 downto 0)	:= x"66";

  function ascii2hex	(arg	: std_logic_vector(7 downto 0))
			return	  std_logic_vector is
  begin
    case arg is
    when ascii1 =>	return "0001";
    when ascii2 =>	return "0010";
    when ascii3 =>	return "0011";
    when ascii4 =>	return "0100";
    when ascii5 =>	return "0101";
    when ascii6 =>	return "0110";
    when ascii7 =>	return "0111";
    when ascii8 =>	return "1000";
    when ascii9 =>	return "1001";
    when asciia =>	return "1010";
    when asciib =>	return "1011";
    when asciic =>	return "1100";
    when asciid =>	return "1101";
    when asciie =>	return "1110";
    when asciif =>	return "1111";
    when others =>	return "0000";
    end case;
  end function ascii2hex;

  signal byteRx		: std_logic_vector(7 downto 0);
  signal byteRxEn	: std_logic;
  signal byteTx		: std_logic_vector(7 downto 0);
  signal byteTxReq	: std_logic;
  signal byteTxAck	: std_logic;
  signal txLoc		: std_logic;
  signal aclr		: std_logic;
  signal rdReq		: std_logic;
  signal empty		: std_logic;
begin
  -- disable unused hardware
  ---------------------------------------------------------------------------
  dramCsN	<= '1';
  epcsCsN	<= '1';
  gSensorCs	<= '0';
  adcCsN	<= '1';

  -- component instantitions
  ---------------------------------------------------------------------------
  rxI: serRx
  generic map (	clkFreq		=> 50000000,
		baudRate	=> baudG)
  port map    (	clk		=> clk50,
		rstN		=> key(1),
		rx		=> rx,		--gpio1(9),	-- crossed Tx
		byte		=> byteRx,
		byteEn		=> byteRxEn);

  txI: serTx
  generic map (	clkFreq		=> 50000000,
		baudRate	=> baudG)
  port map    (	clk		=> clk50,
		rstN		=> key(1),
		byte		=> byteTx,
		byteReq		=> byteTxReq,
		byteAck		=> byteTxAck,
		tx		=> txLoc);	--gpio1(8));	-- crossed Rx

  ledP: process (clk50, key(1)) is
  begin
    if key(1) = '0' then	led	<= (others => '0');
    elsif rising_edge(clk50) then
	if byteRxEn = '1' then	led(7 downto 4)	<= ascii2hex(byteRx);
	end if;
	led(3)	<= byteRxEn;
	led(2)	<= rx;		--gpio1(9);	
	led(1)	<= byteTxReq;
	led(0)	<= txLoc;	--gpio1(8);
    end if;
  end process ledP;

  tx	<= txLoc;
  aclr	<= not key(1);

  fifoI: fifo64 port map(aclr, clk50, byteRx, rdReq, byteRxEn, empty, byteTx);

  fifoP: process (clk50, key(1)) is
  begin
    if key(1) = '0' then
	byteTxReq <= '0';
	rdReq	  <= '0';
    elsif rising_edge(clk50) then
	rdReq	  <= '0';
      if (empty = '0') and (byteTxReq = byteTxAck) then
	byteTxReq <= not byteTxReq;
	rdReq	  <= '1';
      end if;
    end if;
  end process fifoP;

--  echoP: process (clk50, key(1)) is
--    constant	msCntM	: positive := (50000*msEchoG)-1;
--
--    type     stateTy	is (rxWait, txWait);
--    variable state	: stateTy;
--    variable msCnt	: natural range 0 to msCntM;
--
--  begin
--    if key(1) = '0' then	state	  := rxWait;
--				byteTxReq <= '0';
--    elsif rising_edge(clk50) then
--      if state = rxWait then
--	if byteRxEn = '1' then	state	  := txWait;
--				msCnt	  := msCntM;
--				byteTx	  <= byteRx;
--	end if;
--      elsif msCnt = 0 then
--	if byteTxReq = byteTxAck then
--				state	  := rxWait;
--				byteTxReq <= not byteTxReq;
--	end if;
--      else			msCnt	  := msCnt-1;
--      end if;
--    end if;
--  end process echoP;

end architecture wrapper;
--------------------------------------------------------------------------------
-- de0Board.vhd - end
