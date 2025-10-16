include "std.s"

entry:
    -- call fib(25)
    mov  $2,  25
    jsr  fib

    halt

fib:
    -- save calle saved registers
    push $3
    push $2

    -- call fib(n - 1)
    subi $2, $2, 1
    call fib

    -- save result
    mov  $3, $1

    -- call fib(n - 2)
    subi $2, $2, 1
    call fib

    -- add fib(n - 1) and fib(n - 2)
    addi $1, $1, $3

    -- restore calle saved registers
    pop  $2
    pop  $3
    ret