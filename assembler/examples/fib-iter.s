entry:
    mov $1, 0
    mov $2, 1
    mov $15, 5

.fib.loop:
    eq $3, $15, $0
    br $3, .fib.done

    add $3, $1, $2
    copy $1, $2
    copy $2, $3

    jmp .fib.loop

.fib.done:
    halt
