-- serTx.vhd
--
-- entity	serTx		-serial transmitter
--				 Altera CycloneIII 3c25 daughter board
-- architecture	behaviour	-RS232 transmitter, 1-start bit, half-stop
--				-usage:	byteReq = byteAck	-> idle 
--					byteReq toggle		-> request
--					byteAck follows byteReq	-> acknowledge
--			_____     ___ ___ ___ ___ ___ ___ ___ ___ ______
--		rx	     \___+___+___+___+___+___+___+___+___/
--			     start 0   1   2   3   4   5   6   7  stop
--			_____
--		byte	_____+..........................................
--			... ____________________________________________
--		byteReq ___/............................................
--			....................................... ________
--		byteAck _______________________________________/........
--
--------------------------------------------------------------------------------
library ieee;
use ieee.std_logic_1164.all;

-- entity	----------------------------------------------------------------
--------------------------------------------------------------------------------
entity serTx is
generic(clkFreq		: positive	:= 50000000;	-- clock Frequency
	baudRate	: positive	:=    57600);	-- baud rate
port (	clk		: in  std_logic;		-- clock	[R]
	rstN		: in  std_logic;		-- reset	[L]
	byte		: in  std_logic_vector(7 downto 0);	-- data
	byteReq		: in  std_logic;		-- data request	[R+F]
	byteAck		: out std_logic;		-- data acknow.	[R+F]
	tx		: out std_logic);		-- tx line
end entity serTx;

-- architecture	----------------------------------------------------------------
--------------------------------------------------------------------------------
architecture behaviour of serTx is
  signal	dataAck		: std_logic;
begin
  byteAck <= dataAck;

  fsmP:	process (rstN, clk) is
    constant	maxHalf		: positive
				:= integer(real(clkFreq)*0.5/real(baudRate))-1;
    constant	maxOne		: positive
				:= integer(real(clkFreq)/real(baudRate))-1;
    type	stateTy		is (idle, startBit, dataBit);
    variable	state		: stateTy;
    variable	bitCnt		: natural range 0 to 7;
    variable	clkCnt		: natural range 0 to maxOne;
    variable	dataReg		: std_logic_vector(7 downto 0);
  begin
    if rstN = '0' then
	state	:= idle;
	clkCnt	:= 0;
--	dataAck	<= byteReq;
	dataAck	<= '0';
	tx	<= '1';
    elsif rising_edge(clk) then
	case state is
	when idle	=>	-- idle		--------------------------------
	  if clkCnt > 0 then			-- timeout?
		clkCnt  := clkCnt-1;
	  elsif byteReq /= dataAck then		-- request has toggled?
		tx	<= '0';			--   send start bit: '0'
		clkCnt	:= maxOne;		--   init time: 1 #ticks
		dataReg	:= byte;		--   store byte
		state	:= startBit;		--
	  end if;	-- timeout?
	  dataAck	<= byteReq;

	when startBit =>	-- startBit	--------------------------------
	  if clkCnt > 0	then			-- timeout?
		clkCnt	:= clkCnt-1;
	  else	tx	<= dataReg(0);		-- send bit
		dataReg	:= '0' & dataReg(7 downto 1);	-- shift dataReg
		bitCnt	:= 7;			-- init bitCnt
		clkCnt	:= maxOne;		-- init time: 1 #ticks
		state	:= dataBit;		--
	  end if;	-- timeout?

	when dataBit =>		-- dataBit	--------------------------------
	  if clkCnt > 0	then			-- timeout?
		clkCnt	:= clkCnt-1;
	  elsif bitCnt = 0 then			-- whole byte send?
		tx	<= '1';			--   stop bit
		dataAck	<= byteReq;
		clkCnt	:= maxHalf;		--   init time: 1/2 #ticks
		state	:= idle;		--   set idle
	  else	tx	<= dataReg(0);		-- send bit
		dataReg	:= '0' & dataReg(7 downto 1);	-- shift dataReg
		bitCnt	:= bitCnt-1;		--   next bit
		clkCnt	:= maxOne;		--   init time: 1 #ticks
	  end if;	-- timeout?
	end case;	-- state
    end if;		-- rising_edge(clk)
  end process fsmP;

end architecture behaviour;
--------------------------------------------------------------------------------
-- serTx.vhd	- end
