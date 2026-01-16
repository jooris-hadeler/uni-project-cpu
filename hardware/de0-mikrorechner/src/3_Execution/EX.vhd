library ieee; 
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;

entity EX is 
    port (
        imm, pc, alu_val, reg_val: in std_logic_vector(31 downto 0);
        alu_op, rt, rd: in std_logic_vector(4 downto 0);
        clk, reg_dest, reg_write_EX, alu_src, pc_src, mem_write, mem_to_reg_EX, jr, jar: in std_logic; -- mux_sel für alu, write_sel für befehls_mux unten bild            
        pc_out, out_result, data: out std_logic_vector(31 downto 0);
        write_reg: out std_logic_vector(4 downto 0);
        mem_write_out, mem_to_reg_MEM, reg_write_MEM, pc_src_MEM : out std_logic);
end entity EX;

architecture behaviour of EX is
    component alu is
    port( opA, opB: in signed(31 downto 0);
         result: out  signed(31 downto 0);
		 op: in STD_LOGIC_VECTOR(4 downto 0);
		 zero: out STD_LOGIC);
    end component;

    signal alu_result, mux_val: signed(31 downto 0);
    signal zero : STD_LOGIC;

    begin
        aluI: alu	port map (signed(alu_val), mux_val, alu_result, alu_op, zero);

            mux_val <= signed(reg_val) when alu_src = '0' else signed(imm);
            ex_seg_process : process (clk) is
            begin 
                if rising_edge(clk) then

                    if pc_src = '1' then
                        if zero = '1' then
                            pc_src_MEM <= '1';
                        else 
                            pc_src_MEM <= '0';
                        end if;
                    else 
                        pc_src_MEM <= '0';
                    end if;

                    pc_out <= std_logic_vector(signed(pc) + signed(imm)); -- adder und shifter (imm -> offset)

                    if reg_dest = '1' then
                        write_reg <= rd;
                    else 
                        write_reg <= rt;
                    end if;

                    if jr = '1' then
                        pc_out <= alu_val;
                    end if;

                    
                    reg_write_MEM <= reg_write_EX;
                    mem_to_reg_MEM <= mem_to_reg_EX;
                    mem_write_out <= mem_write;
                    data <= reg_val;
                    if jar = '1' then
                        out_result <= pc; -- ergebnis der alu wird 'ausgegeben'
                    else
                        out_result <= std_logic_vector(alu_result); -- ergebnis der alu wird 'ausgegeben'
                    end if;
                    
                end if;
        end process ex_seg_process;

end behaviour;
