entry:
    mov $1, 1

loop:
    nop
    add $1, $1, $1

    jmp loop
    
    nop 
    nop
    add $1, $1, $1
