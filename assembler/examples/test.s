include "std.s"

entry:
    mov  $1, 0xBEEF
    push $1
    mov  $1, 0xDEAD
    push $1

    halt