library ieee; 
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
-- use work.memPkg.all;        -- Import des gesamten Packages memPkg

entity instF is -- Schnittstelle des Instruction-Fetch
    port (
        clk, pc_src : in std_logic := '0'; -- Takt-Signal, Steuersignal f�r Sprung
        rstN : in std_logic := '1';
        iAddr : out STD_LOGIC_VECTOR(9 downto 0);
        iData : in STD_LOGIC_VECTOR(31 downto 0);
        pc_IF : in std_logic_vector(31 downto 0) := "00000000000000000000000000000000"; -- Eingabe des aktuellen PC Counts
        pc_ID: out std_logic_vector(31 downto 0); -- Ausgabe des inkrementierten PC Counts
        instruction: out std_logic_vector (31 downto 0) -- Die gelesene Instruction aus dem ROM
    );
end instF;

architecture behaviour of instF

is 
    signal pc : std_logic_vector(31 downto 0) := "00000000000000000000000000000000";
    signal next_pc : std_logic_vector(31 downto 0);
    
    begin

        next_pc <= pc_IF when pc_src = '1' else STD_LOGIC_VECTOR(unsigned(pc) + 1);
        --pc <= "00000000000000000000000000000000" when rstN = '0';   
        iAddr <= pc(9 downto 0);    
            instf_seg_process : process (clk) is  
            begin
                if rising_edge(clk) then --check ob Flanke von clk = 0 -> 1
                    instruction <= iData; --instruction wird auf den aus ROM geladenen Befehl gesetzt
                    pc_ID <= STD_LOGIC_VECTOR(signed(pc) + 4);
                    pc <= next_pc;
                end if;
        end process instf_seg_process;
end behaviour;

