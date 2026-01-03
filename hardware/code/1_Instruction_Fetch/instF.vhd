library ieee; 
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use work.memPkg.all;        -- Import des gesamten Packages memPkg

entity instF is -- Schnittstelle des Instruction-Fetch
    port (
        pc_in : in std_logic_vector(31 downto 0); -- Eingabe des aktuellen PC Counts
        pc_out: out std_logic_vector(31 downto 0); -- Ausgabe des inkrementierten PC Counts
        instruction: out std_logic_vector (31 downto 0);-- Die gelesene Instruction aus dem ROM
        pc_src: in std_logic; -- Steuersignal für Sprung
        clk : in std_logic -- Takt-Signal
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

    signal addrin : std_logic_vector(15 downto 0); --Leitung for pc_in zu ROM 
    signal instruction_mem : std_logic_vector(31 downto 0); --Leitung for ROM zu instruction
    signal fileIO  : fileIoT := none;
    signal pc: std_logic_vector(31 downto 0) := "00000000000000000000000000000000";
    
    begin
        romI: rom   
            generic map (
            addrWd => 16, --ROM Initialisierung 
            dataWd => 32,
            fileId => "memoryrom.dat"
            ) 
             
            port map (
                addr => addrin,              -- Adresseingang des ROMs
                data => instruction_mem,  -- Datenausgang des ROMs
                fileIO => fileIO
            );
        
        init_mem : process
        begin
            fileIO <= load;
            wait for 5 ns; 
            fileIO <= none;
            wait;
        end process;

        instf_seg_process : process (clk) is
        variable result : unsigned(31 downto 0);
        
        begin
            if rising_edge(clk) then --check ob Flanke von clk = 0 -> 1
                if pc_src = '1' then
                    addrin <= pc_in(15 downto 0); --Nutzt die unteren 16 bits als adresse for pc 
                    result := unsigned(pc_in) + 1;
                else
                    addrin <= pc(15 downto 0);
                    result := unsigned(pc) + 1;
                end if;
                pc <= std_logic_vector(result); --increase pc count um 1
                pc_out <= std_logic_vector(result); --increase pc count um 1
                instruction <= instruction_mem; --instruction wird auf den aus ROM geladenen Befehl gesetzt
                
            end if;
        end process instf_seg_process;
end behaviour;

