.global _start

_start:

    # run only one instance
    csrr    t0, mhartid
    bnez    t0, forever

    # prepare for the loop
    li      s1, 0x10000000  # output offset
    la      s2, hello       # load string start addr into s2
    addi    s3, s2, 13      # set up string end addr in s3

loop:
    lb      s4, 0(s2)       # load next byte at s2 into s4
    sb      s4, 0(s1)       # write byte to output
    addi    s2, s2, 1       # increase s2
    addi    s1, s1, 1       # increase s1
    blt     s2, s3, loop    # branch back until end addr (s3) reached

forever:
    wfi
    j       forever


.section .data

hello:
    .string "hello world!\n"