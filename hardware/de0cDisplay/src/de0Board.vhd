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
use work.cDispPkg.all;

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
--	gpio1		: inout	std_logic_vector(33 downto 0);
--	gpio1In		: in	std_logic_vector( 1 downto 0);

	-- 2x13 GPIO	--------------------------------------------------------
	--	right	DE0-UserManual			=> page 21
--	gpio2		: inout	std_logic_vector(12 downto 0);
--	gpio2In		: in	std_logic_vector( 2 downto 0));

	butWh		: in	std_logic_vector(1 to 8); -- [H]  gpio1(24..31)
	butBk		: in	std_logic_vector(1 to 2); -- [L]  gpio1(16..17)
	butRd		: in	std_logic_vector(1 to 2); -- [L]  gpio1(19..20)

	s_ceN		: out	std_logic;	-- SPI client ena.	[L]
						-- 3-SCE	= gpio1(0)
	s_rstN		: out	std_logic;	-- SPI reset		[L]
						-- 4-RST	= gpio1(1)
	s_dNc		: out	std_logic;	-- SPI data [1]/ctrl [0]
						-- 5-D/C	= gpio1(2)
	s_din		: out	std_logic;	-- SPI data in
						-- 6-DN(MOSI)	= gpio1(3)
	s_clk		: out	std_logic;	-- SPI clock
						-- 7-SCLK	= gpio1(4)
	bgLed		: out	std_logic);	-- background LED
						-- 8-LED	= gpio1(5)
end entity de0Board;


-- architecture	----------------------------------------------------------------
--------------------------------------------------------------------------------
architecture wrapper of de0Board is
  type   stateTy	is (idle, dOn, dOff, dNorm1, dClear1, dAllChars,
				dInv, dNorm2, dClear2, dXY, dString, halt);
  signal state		: stateTy;
  signal clk, clkN	: std_logic;
  signal req, ack	: std_logic;
  signal cmd		: cmdTy;
  signal char		: character;
  signal invC		: std_logic;
  signal xPos		: natural range 0 to 13;
  signal yPos		: natural range 0 to  5;

begin
  -- disable unused hardware
  ------------------------------------------------------------------------------
  dramCsN	<= '1';
  epcsCsN	<= '1';
  gSensorCs	<= '0';
  adcCsN	<= '1';

  -- component instantitions
  ------------------------------------------------------------------------------
  pllI:	pllClk	port map (clk50, clk,  clkN, open, open);	-- 2 MHz clock
--pllI:	pllClk	port map (clk50, open, open, clk,  clkN);	-- 1 MHz clock

  dispI: cDisp14x6
	generic map (	bgLight	=> false)
	port map (	clk	=> clk,
			clkN	=> clkN,
			rstN	=> key(1),
			req	=> req,
			cmd	=> cmd,
			char	=> char,
			invC	=> invC,
			xPos	=> xPos,
			yPos	=> yPos,
			ack	=> ack,
			s_ceN	=> s_ceN,
			s_rstN	=> s_rstN,
			s_dNc	=> s_dNc,
			s_din	=> s_din,
			s_clk	=> s_clk,
			bgLed	=> bgLed);

  demoP: process (clk, key(1)) is
    constant	timeC	: natural	:= 2000000;	--~1sec @2MHz
--  constant	timeC	: natural	:= 1000000;	--~1sec @1MHz
    variable	timer	: natural range 0 to timeC;	--timer, delay states
    constant	stringC	: string	:= "Fertig!";	--test for string output
    variable	strReg	: string(1 to stringC'length);	--string register
    variable	strCnt	: natural range 0 to stringC'length;

    -- state transition		------------------------------------------------
    procedure sTransition (	nxtState	: stateTy;
				command		: cmdTy) is
    begin
	  req	<= not req;
	  state	<= nxtState;
	  cmd	<= command;
    end procedure sTransition;

    procedure sTransition (	nxtState	: stateTy;
				command		: cmdTy;
				timeout		: natural range 0 to timeC) is
    begin
	  req	<= not req;
	  state	<= nxtState;
	  cmd	<= command;
	  timer	:= timeout;
    end procedure sTransition;

    -- timed state transition	------------------------------------------------
    procedure tTransition (	nxtState	: stateTy;
				command		: cmdTy) is
    begin
	if timer = 0 then
	  if ack = req	then	sTransition(nxtState, command);
	  end if;
	else			timer	:= timer-1;
	end if;
    end procedure tTransition;

    procedure tTransition (	nxtState	: stateTy;
				command		: cmdTy;
				timeout		: natural range 0 to timeC) is
    begin
	if timer = 0 then
	  if ack = req	then	sTransition(nxtState, command, timeout);
	  end if;
	else			timer	:= timer-1;
	end if;
    end procedure tTransition;

  begin
    if key(1) = '0' then	-- async. reset			----------------
	state	<= idle;
	req	<= '0';
	char	<= 'X';
	invC	<= '0';
	xPos	<= 0;
	yPos	<= 0;
	timer	:= 0;
    elsif rising_edge(clk) then
      case state is

      -- init. internal vars, wait for ack='0'		------------------------
      when idle		=>
	req	<= '0';
	strReg	:= stringC;
	strCnt	:= stringC'length;
	if ack = '0' then	sTransition(dOn, dispAllOn, timeC);
	end if;

      -- all pixel on,  for timeC			------------------------
      when dOn		=>	tTransition(dOff, dispAllOff, timeC);

      -- all pixel off, for timeC			------------------------
      when dOff		=>	tTransition(dNorm1, dispNormal);

      -- normal display, no timeout			------------------------
      when dNorm1	=>
	if ack = req	then	sTransition(dClear1, dispClear);
	end if;

      -- clear display					------------------------
      when dClear1	=>
	if ack = req	then	sTransition(dAllChars, dispChar, timeC/4);
				char	<= character'left;
	end if;

      -- display char-set: 0..255, timeC/4 between chars	----------------
      when dAllChars	=>
	if timer = 0 then
	  if ack = req	then
	    if char = character'right
			  then	sTransition(dInv, dispInverse, timeC);
			  else	sTransition(dAllChars, dispChar, timeC/4);
				char	<= character'succ(char);
	    end if;
	  end if;
	else			timer	:= timer-1;
	end if;

      -- invert display, for timeC			------------------------
      when dInv		=>	tTransition(dNorm2, dispNormal, timeC);

      -- normal display, no timeout			------------------------
      when dNorm2	=>
	if ack = req	then	sTransition(dClear2, dispClear);
	end if;

      -- clear display					------------------------
      when dClear2	=>
	if ack = req	then	sTransition(dXY, dispPosXY);
				xPos	<= 4;
				yPos	<= 2;
	end if;

      -- set xPos=4, yPos=2				------------------------
      when dXY		=>
	if ack = req	then	sTransition(dString, dispChar);
--				invC	<= '1';
				char	<= strReg(1);
				strReg	:= strReg(2 to strReg'right)&' ';
				strCnt	:= strCnt-1;
	end if;

      -- string output, no timeout			------------------------
      when dString	=>
	if ack = req	then
	  if strCnt = 0 then	state	<= halt;
				invC	<= '0';
			else	sTransition(dString, dispChar);
				if strCnt = 1 then
				  invC	<= '1';
				end if;
                                char	<= strReg(1);
				strReg	:= strReg(2 to strReg'right)&' ';
				strCnt	:= strCnt-1;
	  end if;
	end if;

      -- ...done					------------------------
      when halt		=>	if butRd(1) = '0' then
				  sTransition(idle, dispPosXY);
				  xPos	<= 0;
				  yPos	<= 0;
				end if;
      end case;
    end if;
  end process demoP;

  led <= butWh;
--led(7 downto 4)   <=	butWh(1 to 4);
--led(3 downto 2)   <=	butBk;
--led(1 downto 0)   <=	butRd;

--  with state select					-- some debug output
--    led <=	x"01"	when idle,
--		x"02"	when dOn,
--		x"03"	when dOff,
--		x"04"	when dNorm1,
--		x"05"	when dClear1,
--		x"06"	when dAllChars,
--		x"07"	when dInv,
--		x"08"	when dNorm2,
--		x"09"	when dClear2,
--		x"0a"	when dXY,
--		x"0b"	when dString,
--		x"f0"	when halt;

end architecture wrapper;
--------------------------------------------------------------------------------
-- de0Board.vhd - end
