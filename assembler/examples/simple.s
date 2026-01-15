entry:
    mov $1, 0
    mov $2, 420
    call write

    mov $1, 1
    mov $2, 1337
    call write

    halt

write:
    store $1, $2
    ret