// Allows for adding immediates and registers.
macro addi(dst: reg, src: reg, value: num) {
    liv $30, %value
    add %dst, %src, $30
}

macro subi(dst: reg, src: reg, value: num) {
    liv $30, %value
    sub %dst, %src, $30
}

macro push(src: reg) {
    subi  $31, 1
    store $31, %src
}

macro pop(dst: reg) {
    load %dst, $31
    addi $31, 1
}

macro liv(dst: reg, value: num) {
    lhi %dst, (%value >> 16)
    llo %dst, (%value & 255)
}

macro call(ptr: num) {
    push ret_addr
    liv  $30, %ptr
    jmp  $30
ret_addr:
}

macro ret() {
    pop $30
    jmp $30
}