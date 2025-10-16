-- add an immediate to a register
-- uses register $28 internally
macro addi 3
    mov $28, %3
    add %1, %2, $28
end

-- subtract an immediate from a register
-- uses register $28 internally
macro subi 3
    mov $28, %3
    sub %1, %2, $28
end

-- push a register onto the stack
macro push 1
    subi $sp, $sp, 1
    str $sp, %1
end

-- pop a register from the stack
macro pop 1
    ldr %1, $sp
    addu $sp, $sp, 1
end

-- jump to a subroutine
macro jsr 1
    addi $28, $ip, 3 -- skip the add instruction and the expanded push
    push $28
    jmp %1
end

-- return from a subroutine
macro ret 0
    pop $28
    jmp $28
end

-- enter a new stack frame
macro enter 1
    push $bp
    mov  $bp, $sp
    subi $sp, %1
end

-- leave a stack frame
macro leave 0
    mov $sp, $bp
    pop $bp
end