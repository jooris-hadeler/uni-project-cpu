macro push(src: reg) {
    subi  $31, $31, 1
    store $31, %src
}

macro sim(dst: reg, value: num) {
    slo %dst, %value & 65536
    shi %dst, %value >> 16
}