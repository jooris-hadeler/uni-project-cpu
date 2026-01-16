-- serRx.vhd
--
-- entity	serRx		-serial receiver
--				 Altera CycloneIII 3c25 daughter board
-- architecture	behaviour	-RS232 receiver, 1-start bit, half-stop
--				-new byte signal: byteEn [H]
--			_____     ___ ___ ___ ___ ___ ___ ___ ___ ______
--		rx	     \___+___+___+___+___+___+___+___+___/
--			     start 0   1   2   3   4   5   6   7  stop
--			                                        ________
--		byte	.......................................+________
--			                                        _
--		byteEn	_______________________________________/ \______
--
--------------------------------------------------------------------------------
library ieee;
use ieee.std_logic_1164.all;

-- entity	----------------------------------------------------------------
--------------------------------------------------------------------------------
entity serRx is
generic(clkFreq		: positive	:= 50000000;	-- clock Frequency
	baudRate	: positive	:=    57600);	-- baud rate
port (	clk		: in  std_logic;		-- clock	[R]
	rstN		: in  std_logic;		-- reset	[L]
	rx		: in  std_logic;		-- rx line
	byte		: out std_logic_vector(7 downto 0);	-- data
	byteEn		: out std_logic);		-- data enable	[H]
end entity serRx;

-- architecture	----------------------------------------------------------------
--------------------------------------------------------------------------------
architecture behaviour of serRx is
  signal	byteReg	: std_logic_vector(7 downto 0);
begin
  byte	<= byteReg;

  fsmP:	process (rstN, clk) is
    constant	maxOneHalf	: positive
				:= integer(real(clkFreq)*1.5/real(baudRate))-1;
    constant	maxOne		: positive
				:= integer(real(clkFreq)/real(baudRate))-1;
    type	stateTy		is (idle, getBit);
    variable	state		: stateTy;
    variable	bitCnt		: natural range 0 to 7;
    variable	clkCnt		: natural range 0 to maxOneHalf;
  begin
    if rstN = '0' then
	state	:= idle;
	clkCnt	:= 0;
	byteEn	<= '0';
    elsif rising_edge(clk) then
	case state is
	when idle	=>	-- idle		--------------------------------
	  byteEn <= '0';			-- no new data
	  if clkCnt > 0 then			-- timeout?
		clkCnt  := clkCnt-1;
	  elsif rx = '0' then			-- start bit
		state	:= getBit;		--
		bitCnt	:= 7;			--   init bitCnt
		clkCnt	:= maxOneHalf;		--   init time: 3/2 #ticks
	  end if;	-- timeout?

	when getBit =>		-- getBit	--------------------------------
	  if clkCnt > 0	then			-- timeout?
		clkCnt	:= clkCnt-1;
	  else	byteReg	<= rx & byteReg(7 downto 1);	-- sample bit
		clkCnt	:= maxOne;		--   init time: #ticks
		if bitCnt = 0 then		--   last bit?
			state  := idle;		--     set idle
			byteEn <= '1';		--     complete byte received
		else	bitCnt := bitCnt-1;
		end if;
	  end if;	-- timeout?
	end case;	-- state
    end if;		-- rising_edge(clk)
  end process fsmP;

end architecture behaviour;
--------------------------------------------------------------------------------
-- serRx.vhd	- end
