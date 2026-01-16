-- procTest.vhd
--
-- entity	procTest		-testbench for pipeline processor
-- architecture	testbench		-
--
-- to do:	- replace pipeProc with new top-level design
--		- add component declaration or provide 'procPkg.vhd' with it
--		- write: 'instMem.dat' + 'dataMem.dat'
------------------------------------------------------------------------------
library ieee;						-- packages:
use ieee.std_logic_1164.all;				--   std_logic
use ieee.numeric_std.all;				--   (un)signed
use work.memPkg.all;					--   ramIO / rom
--use work.procPkg.all;					--   pipeProc

-- entity	--------------------------------------------------------------
------------------------------------------------------------------------------
entity procTest is
generic(clkPeriod	: time		:= 20 ns;	-- clock period
	clkCycles	: positive	:= 100);	-- clock cycles
end entity procTest;


-- architecture	--------------------------------------------------------------
------------------------------------------------------------------------------
architecture testbench of procTest is
  signal clk, nRst	: std_logic;
  signal const1		: std_logic;
  signal dnWE		: std_logic;
  signal iAddr,  dAddr	: std_logic_vector( 9 downto 0);  -- 10-bit address!!!
  signal iDataI, dDataI	: std_logic_vector(31 downto 0);  -- mem  => proc
  signal dummy,  dDataO	: std_logic_vector(31 downto 0);  -- proc => mem
  signal iCtrl,  dCtrl	: fileIoT;

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

  component rom is
    generic (
        addrWd	: integer range 2 to 16 := 10;	-- anzahl der speicher bereiche 2^16
		dataWd	: integer range 2 to 32 := 32;	-- bitbreite pro speicheradresse 32bit
		fileId	: string  := "memoryrom10.dat");	-- filename

    port (--	nCS	: in    std_logic;		-- not Chip Select 
	        addr	: in    std_logic_vector(9 downto 0);-- Eingabeadresse des ROMs
	        data	: out	std_logic_vector(31 downto 0);-- Ausgabeadresse des ROMs
	        fileIO	: in	fileIoT := none
         );
    end component;

    component ramIO is
      generic (
          addrWd	: integer range 2 to 16	:= 10;	-- #address bits
      dataWd	: integer range 2 to 32	:= 32;	-- #data    bits
      fileId	: string  := "memoryram10.dat"
      );
      port (--	nCS	: in    std_logic;		-- not Chip   Select
      nWE	: in    std_logic;		-- not Write  Enable
          addr	: in    std_logic_vector(addrWd-1 downto 0);
          dataI	: in	std_logic_vector(dataWd-1 downto 0);
          dataO	: out	std_logic_vector(dataWd-1 downto 0);
          fileIO	: in	fileIoT	:= none);
    end component;

begin
-- sample usage: ramIO instaed of rom	--------------------------------------
--const1 <= '1';
--dummy  <= (others => '-');
--
--instMemI: ramIO	generic map (	addrWd	=> 10,
--					dataWd	=> 32,
--					fileId	=> "instMem.dat")
--			port map    (	nWE	=> const1,	-- read-only
--					addr	=> iAddr,
--					dataI	=> dummy,
--					dataO	=> iDataI,
--					fileIO	=> iCtrl);

  -- memories		------------------------------------------------------
  instMemI: rom		generic map (	addrWd	=> 10,
					dataWd	=> 32,
					fileId	=> "memoryrom10.dat")
			port map    (	addr	=> iAddr,
					data	=> iDataI,
					fileIO	=> iCtrl);
  dataMemI: ramIO	generic map (	addrWd	=> 10,
					dataWd	=> 32,
					fileId	=> "memoryram10.dat")
			port map    (	nWE	=> dnWE,
					addr	=> dAddr,
					dataI	=> dDataO,
					dataO	=> dDataI,
					fileIO	=> dCtrl);

  -- pipe processor	------------------------------------------------------
  pipeProcI: Prozessor	port map    (	clk	=> clk,
					rstN	=> nRst,
					iAddr	=> iAddr,
					iData	=> iDataI,
					dnWE	=> dnWE,
					dAddr	=> dAddr,
					dDataI	=> dDataI,
					dDataO	=> dDataO);

  -- stimuli		------------------------------------------------------
  stiP: process is
  begin
    clk		<= '0';
    nRst	<= '0',   '1'  after 5 ns;
    iCtrl	<= load,  none after 5 ns;
    dCtrl	<= load,  none after 5 ns;
    wait for clkPeriod/2;
    for n in 1 to clkCycles loop
	clk <= '0', '1' after clkPeriod/2;
	wait for clkPeriod;
    end loop;
    wait;
  end process stiP;

end architecture testbench;
------------------------------------------------------------------------------
-- procTest.vhd	- end
