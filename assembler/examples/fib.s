include "std.s"

define A $1
define B $2
define T $3

define ITER $5
define LIMIT $6

fib:
    mov A, 0
    mov B, 1
    mov ITER, 0
    mov LIMIT, 25

loop:
    add T, A, B
    mov A, B
    mov B, T

    addi ITER, ITER, 1

    ltu $28, ITER, LIMIT
    br $28, loop

done:
    halt