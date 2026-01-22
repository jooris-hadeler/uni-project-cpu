entry:
    mov $1, 0
    mov $2, 1
    mov $3, 1
    mov $15, 5

.fib.loop:
    -- if $15 != 0
    br $15, $0, .fib.done

    -- $1, $2 = $2, $1 + $2
    add $4, $1, $2
    copy $1, $2
    copy $2, $4

    -- $15 = $15 - 1
    sub $15, $15, $3

    jmp .fib.loop

.fib.done:
    nop
    jmp .fib.done
