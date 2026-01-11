library IEEE;
use IEEE.STD_LOGIC_1164.ALL;
use IEEE.NUMERIC_STD.ALL;

entity instFTest is
    generic(	
        periodC	: time		:= 10 ns;
        cyclesC	: integer	:= 100);
end instFTest;

architecture Behavioral of instFTest is
    -- Komponente instfTest instanziieren
    component instF
        port (
        clk, pc_src : in std_logic := '0'; -- Takt-Signal, Steuersignal für Sprung
        pc_IF : in std_logic_vector(31 downto 0) := "00000000000000000000000000000000"; -- Eingabe des aktuellen PC Counts
        pc_ID: out std_logic_vector(31 downto 0); -- Ausgabe des inkrementierten PC Counts
        instruction: out std_logic_vector (31 downto 0) -- Die gelesene Instruction aus dem ROM
    );
    end component;

    -- Signale fÃ¼r die Testbench
    signal clk         : std_logic := '0';
    signal pc_in       : std_logic_vector(31 downto 0) := (others => '0');
    signal pc_out      : std_logic_vector(31 downto 0);
    signal pc_src      : std_logic := '0';
    signal instruction : std_logic_vector(31 downto 0);

    -- Taktgenerator
begin
    -- Instanz des zu testenden Moduls
    uut: instF
        port map (
            clk         => clk,
            pc_src      => pc_src,
            pc_IF       => pc_in,
            pc_ID      => pc_out,
            instruction => instruction
        );

    -- Taktprozess
    clk_process: process
    begin
        for i in 1 to cyclesC loop
            clk <= '0';
            wait for periodC/2;
            clk <= '1';
            wait for periodC/2;
        end loop;
        wait;
    end process;

    -- Testszenarien
    stimulus_process: process
    begin
        -- Initialisierung
        pc_in <= "00000000000000000000000000000000";
        wait for periodC;

        pc_in <= "00000000000000000000000000000001";
        wait for periodC;

        -- PC erhöhen und prüfen
        pc_src <= '1';
        pc_in <= "00000000000000000000000000000101";
        wait for periodC;

        -- Testende
        wait;
    end process;
end architecture Behavioral;
