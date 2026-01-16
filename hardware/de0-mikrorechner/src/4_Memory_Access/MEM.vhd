library ieee; 
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use work.memPkg.all;
--library work;
-- memory sim referenzen

-- logisches and oder or in dieser stufe recherchierena
-- kontroll signale in allen stufen überprüfen

entity MEM is 
    port (
        pc_in, address_in, write_data : in std_logic_vector(31 downto 0) :="00000000000000000000000000000000";
        clk, mem_write, mem_to_reg_in, reg_write_in, pc_src_in : in std_logic;
        write_reg: in std_logic_vector(4 downto 0);
        read_data_out, adress_out, pc_out: out std_logic_vector(31 downto 0);
        mem_to_reg_WB, reg_write_WB, pc_src_IF : out std_logic;
        write_reg_out: out std_logic_vector(4 downto 0);
        dnWE : out STD_LOGIC;
        dAddr : out STD_LOGIC_VECTOR(9 downto 0);
        dDataI : in STD_LOGIC_VECTOR(31 downto 0) ;
        dDataO : out STD_LOGIC_VECTOR(31 downto 0)
    );
end entity MEM;

architecture behaviour of MEM is

    signal nWE: STD_LOGIC := '1';
    signal read_data: std_logic_vector(31 downto 0);
    signal address : std_logic_vector(15 downto 0);
    signal sel_alu_val, sel_reg_val : std_logic_vector(4 downto 0);
    signal fileIO_in: fileIoT := none;
    
    begin
        dnWE <= not mem_write;
        dAddr <= address_in(9 downto 0);
        dDataO <= write_data;

        pc_out <= pc_in;
        pc_src_IF <= pc_src_in;

        init : process is
        begin
            dAddr <= "0000000000";
            wait;
        end process init;

        mem_seg_process : process (clk) is
        begin
            if rising_edge(clk) then
                adress_out <= address_in;
                mem_to_reg_WB <= mem_to_reg_in;
                reg_write_WB <= reg_write_in;
                read_data_out <= dDataI;
                write_reg_out <= write_reg;
            end if;
        end process mem_seg_process;
end behaviour;
