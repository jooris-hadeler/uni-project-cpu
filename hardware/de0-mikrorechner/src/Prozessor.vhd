library ieee; 
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;

entity Prozessor is 
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
end entity Prozessor;

architecture behaviour of Prozessor is

    component instF is -- Schnittstelle des Instruction-Fetch
    port (
        clk, pc_src : in std_logic := '0'; -- Takt-Signal, Steuersignal f�r Sprung
        rstN : in std_logic := '1';
        iAddr : out STD_LOGIC_VECTOR(9 downto 0);
        iData : in STD_LOGIC_VECTOR(31 downto 0);
        pc_IF : in std_logic_vector(31 downto 0) := "00000000000000000000000000000000"; -- Eingabe des aktuellen PC Counts
        pc_ID: out std_logic_vector(31 downto 0); -- Ausgabe des inkrementierten PC Counts
        instruction: out std_logic_vector (31 downto 0) -- Die gelesene Instruction aus dem ROM
    );
    end component;

    component ID is 
    port (
        pc_in, instruction, write_data : in std_logic_vector(31 downto 0);
        clk, reg_wE :                    in std_logic;
        write_reg :                      in std_logic_vector(4 downto 0);
        pc_out, alu_val, reg_val, imm :  out std_logic_vector(31 downto 0);
        alu_op, rt, rd :                 out std_logic_vector(4 downto 0);

        reg_dest, reg_write_EX, alu_src,
        pc_src, mem_write,
        mem_to_reg_EX, jr, jar :              out std_logic
    );
    end component;

    component EX is
    port (
        imm, pc, alu_val, reg_val: in std_logic_vector(31 downto 0);
        alu_op, rt, rd: in std_logic_vector(4 downto 0);
        clk, reg_dest, reg_write_EX, alu_src, pc_src, mem_write, mem_to_reg_EX, jr, jar : in std_logic; -- mux_sel für alu, write_sel für befehls_mux unten bild            
        pc_out, out_result, data: out std_logic_vector(31 downto 0);
        write_reg: out std_logic_vector(4 downto 0);
        mem_write_out, mem_to_reg_MEM, reg_write_MEM, pc_src_MEM : out std_logic);
    end component;

   component MEM is 
    port (
        pc_in, address_in, write_data : in std_logic_vector(31 downto 0);
        clk, mem_write, mem_to_reg_in, reg_write_in, pc_src_in : in std_logic;
        write_reg: in std_logic_vector(4 downto 0);
        read_data_out, adress_out, pc_out: out std_logic_vector(31 downto 0);
        mem_to_reg_WB, reg_write_WB, pc_src_IF : out std_logic;
        write_reg_out: out std_logic_vector(4 downto 0);
        dnWE : out STD_LOGIC;
        dAddr : out STD_LOGIC_VECTOR(9 downto 0);
        dDataI : in STD_LOGIC_VECTOR(31 downto 0);
        dDataO : out STD_LOGIC_VECTOR(31 downto 0)
    );
    end component;

   component WB is 
    port(
        data_val, alu_val : in std_logic_vector(31 downto 0);
        clk, mem_to_reg_WB, reg_write_WB : in std_logic;
        write_reg_in : in std_logic_vector(4 downto 0);
        write_reg_out : out std_logic_vector(4 downto 0);
        write_enable_out : out std_logic;
        write_data : out std_logic_vector(31 downto 0)
    );
    end component;

    -- signal für instF
    signal pc_IF : std_logic_vector(31 downto 0);
    signal pc_src_IF : STD_LOGIC;
    
    -- signal für ID
    signal pc_ID, instruction_ID, write_data_ID : std_logic_vector(31 downto 0);
    signal reg_wE_ID : std_logic; -- ID
    signal write_reg_ID : std_logic_vector(4 downto 0); -- ID

    -- signal für EX
    signal imm_EX, pc_EX, alu_val_EX, reg_val_EX : std_logic_vector(31 downto 0); -- EX
    signal alu_op_EX, rt_EX, rd_EX : std_logic_vector(4 downto 0);
    signal reg_dest_EX, reg_write_EX, alu_src_EX, pc_src_EX, mem_write_EX, mem_to_reg_EX, jr_EX, jar_EX : std_logic;

     -- Signale für MEM-Stufe
    signal pc_MEM, address_MEM, write_data_MEM : std_logic_vector(31 downto 0);
    signal mem_write_MEM, mem_to_reg_MEM, reg_write_MEM, pc_src_MEM : std_logic;
    signal write_reg_MEM : std_logic_vector(4 downto 0);
    
    -- Signale für WB-Stufe
    signal data_val_WB, alu_val_WB : std_logic_vector(31 downto 0);
    signal mem_to_reg_WB, reg_write_WB : std_logic;
    signal write_reg_WB : STD_LOGIC_VECTOR(4 downto 0);

    begin

    instFI: instF 
    port map (
        clk =>  clk,
        pc_src => pc_src_IF,
        rstN => rstN,
        iAddr => iAddr,
        iData => iData,
        pc_IF => pc_IF,
        pc_ID => pc_ID,
        instruction => instruction_ID
    );
    
    IDI: ID
    port map(
        pc_in => pc_ID,
        instruction => instruction_ID,
        write_data => write_data_ID,
        clk => clk,
        reg_wE =>  reg_wE_ID,
        write_reg => write_reg_ID,                
        pc_out => pc_EX,
        alu_val => alu_val_EX,
        reg_val => reg_val_EX,
        imm => imm_EX,
        alu_op => alu_op_EX,
        rd => rd_EX,
        rt => rt_EX,
        reg_dest => reg_dest_EX,
        reg_write_EX => reg_write_EX,
        alu_src => alu_src_EX,
        pc_src =>pc_src_EX,
        mem_write =>mem_write_EX,
        mem_to_reg_EX =>mem_to_reg_EX,
        jr => jr_EX,
        jar => jar_EX
    );

    EXI: EX
    port map (
        imm         => imm_EX,       
        pc          => pc_EX,     -- aus ID
        alu_val     => alu_val_EX,   -- aus ID
        reg_val     => reg_val_EX,   -- aus ID
        alu_op      => alu_op_EX,    -- aus ID
        rt          => rt_EX,        -- aus ID
        rd          => rd_EX,        -- aus ID
        clk         => clk,
        reg_dest    => reg_dest_EX,
        reg_write_EX => reg_write_EX,
        alu_src => alu_src_EX,
        pc_src => pc_src_EX,
        mem_write   => mem_write_EX,           -- Steuerleitung aus ID
        mem_to_reg_EX   => mem_to_reg_EX,          -- Steuerleitung aus ID
        jr          => jr_EX,
        jar         => jar_EX,
        pc_out        => pc_MEM,     
        out_result          => address_MEM,   -- Steuerleitung
        data          => write_data_MEM,    -- Steuerleitung
        write_reg     => write_reg_MEM,            -- Ausgabe
        mem_write_out  => mem_write_MEM,        -- Ausgabe
        mem_to_reg_MEM   => mem_to_reg_MEM,      
        reg_write_MEM      => reg_write_MEM ,
        pc_src_MEM       => pc_src_MEM 
        );

    MEMI: MEM
        port map (
            pc_in          => pc_MEM,        -- PC aus EX-Stufe
            address_in      => address_MEM, -- Speicheradresse aus ALU-Ergebnis (EX)
            write_data     => write_data_MEM, -- Daten für Speicher (EX-Ergebnis)
            clk            => clk,           -- Takt-Signal
            mem_write         => mem_write_MEM,  -- Speicher-Schreibsignal
            mem_to_reg_in  => mem_to_reg_MEM,    -- Steuersignal für WB-Stufe
            reg_write_in   => reg_write_MEM,     -- Register Write Enable Signal
            pc_src_in       => pc_src_MEM,
            write_reg => write_reg_MEM,
            read_data_out      => data_val_WB,      -- Gelesene Daten für WB
            adress_out     => alu_val_WB,              -- Falls nicht benötigt
            pc_out         => pc_IF,              -- PC für WB
            mem_to_reg_WB => mem_to_reg_WB,     -- Weitergabe an WB
            reg_write_WB  => reg_write_WB,      -- Weitergabe an WB
            pc_src_IF => pc_src_IF,
            write_reg_out => write_reg_WB,
            dnWE => dnWE,
            dAddr => dAddr,
            dDataI => dDataI,
            dDataO => dDataO
            );

    WBI: WB
    port map (
        data_val       => data_val_WB,   -- Speicherwert aus MEM-Stufe
        alu_val        => alu_val_WB,  -- ALU-Ergebnis für WB
        clk            => clk,            -- Takt-Signal
        mem_to_reg_WB  => mem_to_reg_WB,  -- Steuersignal zur Auswahl des WB-Werts
        reg_write_WB   => reg_write_WB,   -- Schreibsignal für Register
        write_reg_in   => write_reg_WB,   -- Zielregister für WB
        write_reg_out  => write_reg_ID,   -- Zielregister bleibt gleich
        write_enable_out => reg_wE_ID, -- Finales Write-Enable-Signal für Register
        write_data     => write_data_ID   -- Finaler Wert für Register-Write
    );

    keep_alive : process
        begin
            wait;
        end process;

end behaviour;