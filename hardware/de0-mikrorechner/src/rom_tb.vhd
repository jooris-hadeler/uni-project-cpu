library IEEE;
use IEEE.STD_LOGIC_1164.ALL;
use IEEE.NUMERIC_STD.ALL;
use work.memPkg.all;

entity rom_tb is
end rom_tb;

architecture test of rom_tb is
    -- Komponente des RAM-Moduls
    component rom
        generic (	addrWd	: integer range 2 to 16	:= 16;
		dataWd	: integer range 2 to 32	:= 32;
		fileId	: string  := "memoryrom.dat"); 
    port ( --	nCS	: in    std_logic;
	        addr	: in    std_logic_vector(addrWd-1 downto 0);
	        data	: out	std_logic_vector(dataWd-1 downto 0);
	        fileIO	: in	fileIoT := none
            );
    end component;

signal addr    : std_logic_vector(15 downto 0) := (others => '0');  
signal data   : std_logic_vector(31 downto 0);  
signal fileIO  : fileIoT := none;  

begin
    -- Instanz des ROM-Moduls
    romI: rom
    generic map (
        addrWd => 16,  
        dataWd => 32,  
        fileId => "memoryrom.dat"
    )
    port map (
        addr  => addr,
        data => data,  
        fileIO => fileIO
    );

    -- Testprozess
    process
    begin
        -- 📌 Warte auf Initialisierung
        wait for 20 ns;

                -- 📌 Speicherinhalt aus Datei `memoryrom.dat` lesen
        fileIO <= load;
        wait for 10 ns;
        fileIO <= none;  -- Zurücksetzen

        -- 📌 Lese den gespeicherten Wert aus Adresse 0x05
        addr   <= "0000000000000101";  
        wait for 10 ns;

        -- 📌 Debugging-Ausgabe
        report "Daten an Adresse 0x05 = " & integer'image(to_integer(unsigned(data)));

        -- 📌 Testende (Fehlerbehebung: `severity note` statt `failure`)
        wait for 50 ns;
        report "Test beendet." severity note;
        wait;
    end process;

end test;
