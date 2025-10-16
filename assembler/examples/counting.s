macro addi 3
    mov $28, %3
    add %1, %2, $28
end

-- macro addi %dst, %src, %imm 
-- begin
--     mov $28, %imm
--     add %dst, %src, $28
-- end

entry:
    mov $0, 0
    mov $1, 0
    mov $2, 0xFF

loop:
    addi $1, $1, 3
    addi $0, $0, 1
    
    cltu $3, $0, $2
    br $3, loop

done:
    halt
