library ieee; 
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use work.opcodes.all;
use work.funct_codes.all;
use work.alu_opcode.all;


entity ID is 
    port (
        pc_in, instruction, write_data : in std_logic_vector(31 downto 0);
        clk, reg_wE :                    in std_logic;
        write_reg :                      in std_logic_vector(4 downto 0);
        pc_out, alu_val, reg_val, imm :  out std_logic_vector(31 downto 0);
        alu_op, rt, rd :                 out std_logic_vector(4 downto 0);

        reg_dest, reg_write_EX, alu_src,
        pc_src, mem_write,
        mem_to_reg_EX, jr :              out std_logic
    );
end entity ID;
architecture behaviour of ID
    is component registerbank is
    port(
        clk :   in std_logic;
        dIn :   in signed(31 downto 0); --input
        dOutA : out signed(31 downto 0); --outputA
        dOutB : out signed(31 downto 0); --outputB
        selA :  in std_logic_vector(5 downto 1); --Registernr für dOutA
        selB :  in std_logic_vector(5 downto 1); --Registernr für dOutB
        selD :  in std_logic_vector(5 downto 1); --Registernr für dIn
        wE :    in std_logic
    );
    end component;

    signal sel_alu_val, sel_reg_val : std_logic_vector(4 downto 0);
    signal alu_val_sig, reg_val_sig : STD_LOGIC_VECTOR(31 downto 0);

    begin
        registerbankI: registerbank	port map (
            clk => clk,
            dIn => signed(write_data),
            std_logic_vector(dOutA) => alu_val_sig,
            std_logic_vector(dOutB) => reg_val_sig,
            selA => instruction(25 downto 21),
            selB => instruction(20 downto 16),
            selD => write_reg,
            wE => reg_wE );
            
            id_seg_process : process (clk) is
            begin
                
                if rising_edge(clk) then
                    
                    --default control signals
                    reg_dest <= '0';
                    reg_write_EX <= '0';
                    alu_src <= '0';
                    pc_src <= '0';
                    mem_write <= '0';
                    mem_to_reg_EX <= '0';
                    jr <= '0';
                    
                    alu_val <= alu_val_sig;
                    reg_val <= reg_val_sig;

                    rt <= instruction(20 downto 16);
                    rd <= instruction(15 downto 11);
                    
                    case instruction(31 downto 26) is
                        when opc_r =>
                        case instruction(5 downto 0) is
                            when funct_add => alu_op <= alu_add;
                            when funct_sub => alu_op <= alu_sub;
                            when funct_and => alu_op <= alu_and;
                            when funct_or => alu_op <= alu_or;
                            when funct_xor => alu_op <= alu_xor;
                            when funct_shl => alu_op <= alu_lsl;
                            when funct_sal => alu_op <= alu_lsl;
                            when funct_shr => alu_op <= alu_lsr;
                            when funct_sar => alu_op <= alu_asr;
                            when funct_not => alu_op <= alu_not;
                            when funct_lts => alu_op <= alu_cmplt;
                            when funct_gts => alu_op <= alu_cmpgt;
                            when funct_ltu => alu_op <= alu_cmplt_u;
                            when funct_gtu => alu_op <= alu_cmpgt_u;
                            when funct_eq => alu_op <= alu_cmpe;
                            when funct_ne => alu_op <= alu_cmpne;
                            when others => alu_op <= alu_add;
                        end case;
                        reg_dest <= '1';
                        reg_write_EX <= '1';
                        when opc_shi => 
                        alu_op <= alu_shi;
                        alu_src <='1';
                        reg_write_EX <= '1';
                        when opc_slo =>
                        alu_op <= alu_slo;
                        alu_src <='1';
                        reg_write_EX <= '1';
                        when opc_load => 
                        alu_op <= alu_add;
                        reg_write_EX <= '1';
                        alu_src <='1';
                        mem_to_reg_EX <= '1';
                        when opc_store => 
                        alu_op <= alu_add;
                        alu_src <='1';
                        mem_write <= '1';
                        when opc_br => 
                        sel_reg_val <= "00000";
                        alu_op <= alu_cmpne;
                        pc_src <= '1';
                        when opc_jr => 
                        sel_reg_val <= instruction(20 downto 16);
                        alu_op <= alu_sub;
                        pc_src <= '1';
                        jr <= '1';
                        when opc_jmp =>
                        sel_alu_val <= "00000";
                        sel_reg_val <= "00000";
                        alu_op <= alu_sub;
                        pc_src <= '1';
                        when opc_jal => 
                        sel_alu_val <= "00000";
                        sel_reg_val <= "00000";
                        alu_op <= alu_sub;
                        reg_write_EX <= '1';
                        pc_src <= '1';
                        when opc_noop => alu_op <= alu_add;
                        when others => alu_op <= alu_add;
                    end case;
                    pc_out  <= pc_in;
                    
                    if instruction(15) = '0' then -- implicit sign extend
                        imm <= "0000000000000000" & instruction(15 downto 0);
                    else 
                        imm <= "1111111111111111" & instruction(15 downto 0);
                    end if;


                end if;    
            end process id_seg_process;
end behaviour;
