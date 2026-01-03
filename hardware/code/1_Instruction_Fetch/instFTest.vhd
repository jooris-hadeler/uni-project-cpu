library IEEE;
use IEEE.STD_LOGIC_1164.ALL;
use IEEE.NUMERIC_STD.ALL;

entity instFTest is
    generic(	
        periodC	: time		:= 2 ns;
        cyclesC	: integer	:= 100);
end instFTest;

architecture Behavioral of instFTest is
    -- Komponente instfTest instanziieren
    component instF
        Port (
            pc_in       : in  std_logic_vector(31 downto 0);
            pc_out      : out std_logic_vector(31 downto 0);
            instruction : out std_logic_vector(31 downto 0);
            pc_src      : in std_logic;
            clk         : in  std_logic
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
            pc_in       => pc_in,
            pc_out      => pc_out,
            pc_src      => pc_src,
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
