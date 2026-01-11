library ieee; 
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use work.memPkg.all;        -- Import des gesamten Packages memPkg

entity instF is -- Schnittstelle des Instruction-Fetch
    port (
        clk, pc_src : in std_logic := '0'; -- Takt-Signal, Steuersignal für Sprung
        pc_IF : in std_logic_vector(31 downto 0) := "00000000000000000000000000000000"; -- Eingabe des aktuellen PC Counts
        pc_ID: out std_logic_vector(31 downto 0); -- Ausgabe des inkrementierten PC Counts
        instruction: out std_logic_vector (31 downto 0) -- Die gelesene Instruction aus dem ROM
    );
end instF;

architecture behaviour of instF

is component rom is
    generic (
        addrWd	: integer range 2 to 16 := 16;	-- anzahl der speicher bereiche 2^16
		dataWd	: integer range 2 to 32 := 32;	-- bitbreite pro speicheradresse 32bit
		fileId	: string  := "memoryrom.dat");	-- filename

    port (--	nCS	: in    std_logic;		-- not Chip Select 
	        addr	: in    std_logic_vector(15 downto 0);-- Eingabeadresse des ROMs
	        data	: out	std_logic_vector(31 downto 0);-- Ausgabeadresse des ROMs
	        fileIO	: in	fileIoT := none
         );
    end component;

    signal instruction_rom : std_logic_vector(31 downto 0); --Leitung for ROM zu instruction
    signal fileIO  : fileIoT := none;
    signal pc : std_logic_vector(31 downto 0) := "00000000000000000000000000000000";
    signal next_pc : std_logic_vector(31 downto 0);
    
    begin

        next_pc <= pc_IF when pc_src = '1' else STD_LOGIC_VECTOR(unsigned(pc) + 1);
            
            
            romI: rom   
            generic map (
                addrWd => 16, --ROM Initialisierung 
                dataWd => 32,
                fileId => "memoryrom.dat"
                ) 
                
                port map (
                    addr => pc(15 downto 0),              -- Adresseingang des ROMs
                    data => instruction_rom,  -- Datenausgang des ROMs
                    fileIO => fileIO
                    );
                    
                    init : process
                begin
                    fileIO <= load;
                    wait for 5 ns; 
                    fileIO <= none;
                    wait;
                end process;
                
            instf_seg_process : process (clk) is
                
            begin
                if rising_edge(clk) then --check ob Flanke von clk = 0 -> 1
                    instruction <= instruction_rom; --instruction wird auf den aus ROM geladenen Befehl gesetzt
                    pc_ID <= STD_LOGIC_VECTOR(signed(pc) + 4);
                    pc <= next_pc;
                end if;
        end process instf_seg_process;
end behaviour;

