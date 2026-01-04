library ieee; 
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;

entity EX is 
    port (
        imm, pc, alu_val, reg_val: in std_logic_vector(31 downto 0);
        alu_op, rt, rd: in std_logic_vector(4 downto 0);
        clk, reg_dest, reg_write_EX, alu_src, pc_src, mem_write, mem_to_reg_EX, jr: in std_logic; -- mux_sel für alu, write_sel für befehls_mux unten bild            
        pc_out, out_result, data: out std_logic_vector(31 downto 0);
        write_reg: out std_logic_vector(4 downto 0);
        mem_write_out, mem_to_reg_MEM, reg_write_MEM : out std_logic);
end entity EX;

architecture behaviour of EX is
    component alu is
    port(
        opA, opB: in signed(31 downto 0);
        result: out  signed(31 downto 0);
		op: in STD_LOGIC_VECTOR(4 downto 0));
    end component;

    signal alu_result, mux_val: signed(31 downto 0);
    signal imm_signed : signed(31 downto 0);

    begin
        aluI: alu	port map (signed(alu_val), mux_val, alu_result, alu_op);

            
            ex_seg_process : process (clk) is
            begin 
                if rising_edge(clk) then
                
                if alu_src = '0' then
                    mux_val <= signed(reg_val);
                else
                    mux_val <= signed(imm);
                end if;

                if pc_src = '1' then 
                    imm_signed <= signed(imm);
                    pc_out <= std_logic_vector(signed(pc) + imm_signed); -- adder und shifter (imm -> offset)
                else 
                    pc_out <= pc;
                end if;

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
                out_result <= std_logic_vector(alu_result); -- ergebnis der alu wird 'ausgegeben'
                
            end if; --weitere speicherwerte einfach mit in process integrieren
        end process ex_seg_process;

end behaviour;
