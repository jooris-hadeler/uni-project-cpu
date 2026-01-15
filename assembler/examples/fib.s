entry:
    mov $9, 3
    call fib
    halt

-- calling conv: fib(n: $9)
fib:
    -- push caller saved registers
    push $1
    push $2
    push $3
    push $4
    push $5

    -- constants
    mov $1, 1
    mov $2, 2

    copy $4, $9

    -- if $4 < 2 goto .fib.ret
    ltu $3, $4, $2
    br $3, .fib.ret

    -- fib(n - 1)
    sub $9, $4, $1
    call fib
    copy $9, $5

    -- fib(n - 2)
    sub $9, $4, $2
    call fib
    add $9, $9, $5

.fib.ret:
    -- pop caller saved registers
    pop $5
    pop $4
    pop $3
    pop $2
    pop $1

    ret