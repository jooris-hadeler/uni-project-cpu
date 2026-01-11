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
        pc_in, address_in, write_data : in std_logic_vector(31 downto 0);
        clk, mem_write, mem_to_reg_in, reg_write_in, pc_src_in : in std_logic;
        write_reg: in std_logic_vector(4 downto 0);
        read_data_out, adress_out, pc_out: out std_logic_vector(31 downto 0);
        mem_to_reg_WB, reg_write_WB, pc_src_IF : out std_logic;
        write_reg_out: out std_logic_vector(4 downto 0)
    );
end entity MEM;

architecture behaviour of MEM is
    component ramIO is
    generic (
        addrWd	: integer range 2 to 16	:= 8;	-- #address bits
		dataWd	: integer range 2 to 32	:= 8;	-- #data    bits
		fileId	: string  := "memory.dat"
    );
    port (--	nCS	: in    std_logic;		-- not Chip   Select
		nWE	: in    std_logic;		-- not Write  Enable
        addr	: in    std_logic_vector(addrWd-1 downto 0);
        dataI	: in	std_logic_vector(dataWd-1 downto 0);
        dataO	: out	std_logic_vector(dataWd-1 downto 0);
        fileIO	: in	fileIoT	:= none);
    end component;

    signal nWE: STD_LOGIC := '1';
    signal read_data: std_logic_vector(31 downto 0);
    signal address : std_logic_vector(15 downto 0);
    signal sel_alu_val, sel_reg_val : std_logic_vector(4 downto 0);
    signal fileIO_in: fileIoT := none;
    
    begin
        pc_out <= pc_in;
        pc_src_IF <= pc_src_in;

        ramIOI: ramIO   generic map (addrWd => 16,
                                     dataWd => 32,
                                     fileID => "memoryram.dat")
                        port map (nWE, address, write_data, read_data, fileIO_in);
        
                            
        address <= address_in(15 downto 0);
        nWE <= NOT mem_write;

        mem_seg_process : process (clk) is
        begin
            if rising_edge(clk) then
                adress_out <= address_in;
                mem_to_reg_WB <= mem_to_reg_in;
                reg_write_WB <= reg_write_in;
                read_data_out <= std_logic_vector(read_data);
                write_reg_out <= write_reg;
                if mem_write = '1' then
                    fileIO_in <= dump;
                else
                    fileIO_in <= load;
                end if;
            end if;
        end process mem_seg_process;
end behaviour;
