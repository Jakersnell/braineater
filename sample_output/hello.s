
.section __DATA, __data
    .p2align 4
    MEM:
        .zero 8192
    
    .p2align 4
    BUF:
        .zero 256

.section    __TEXT,__text
.global _main                             
                                        
.p2align 4
_main:

    ; init buf ptr
    adrp    x9, BUF@PAGE
    add     x9, x9, BUF@PAGEOFF

    ; init mem ptr
    adrp    x10, MEM@PAGE
    add     x10, x10, MEM@PAGEOFF

    ; value register
    mov     w12, #0

        
    strb     w12, [x10]                
    add      x10, x10, #1
    ldrb     w12, [x10]
        
    add     w12, w12, #8           
        
.p2align 4
loop_0:   
    cbz     w12, end_loop_0
            
    strb    w12, [x10]
    sub     x10, x10, #1           
    ldrb    w12, [x10]
        
    add     w12, w12, #9           
        
    strb     w12, [x10]                
    add      x10, x10, #1
    ldrb     w12, [x10]
        
    sub     w12, w12, #1            
        
    b       loop_0
.p2align 4
end_loop_0:  
            
    strb    w12, [x10]
    sub     x10, x10, #1           
    ldrb    w12, [x10]
        
    strb    w12, [x9]
    mov     x0, #1                       
    mov     x1, x9
    mov     x2, #1
    mov     x16, #4
    svc     0
        
    strb     w12, [x10]                
    add      x10, x10, #1
    ldrb     w12, [x10]
        
    add     w12, w12, #4           
        
.p2align 4
loop_1:   
    cbz     w12, end_loop_1
            
    strb    w12, [x10]
    sub     x10, x10, #1           
    ldrb    w12, [x10]
        
    add     w12, w12, #7           
        
    strb     w12, [x10]                
    add      x10, x10, #1
    ldrb     w12, [x10]
        
    sub     w12, w12, #1            
        
    b       loop_1
.p2align 4
end_loop_1:  
            
    strb    w12, [x10]
    sub     x10, x10, #1           
    ldrb    w12, [x10]
        
    add     w12, w12, #1           
        
    strb    w12, [x9]
    mov     x0, #1                       
    mov     x1, x9
    mov     x2, #1
    mov     x16, #4
    svc     0
        
    add     w12, w12, #7           
        
    strb    w12, [x9]
    mov     x0, #1                       
    mov     x1, x9
    mov     x2, #1
    mov     x16, #4
    svc     0
        
    strb    w12, [x9]
    mov     x0, #1                       
    mov     x1, x9
    mov     x2, #1
    mov     x16, #4
    svc     0
        
    add     w12, w12, #3           
        
    strb    w12, [x9]
    mov     x0, #1                       
    mov     x1, x9
    mov     x2, #1
    mov     x16, #4
    svc     0
        
    strb     w12, [x10]                
    add      x10, x10, #2
    ldrb     w12, [x10]
        
    add     w12, w12, #6           
        
.p2align 4
loop_2:   
    cbz     w12, end_loop_2
            
    strb    w12, [x10]
    sub     x10, x10, #1           
    ldrb    w12, [x10]
        
    add     w12, w12, #7           
        
    strb     w12, [x10]                
    add      x10, x10, #1
    ldrb     w12, [x10]
        
    sub     w12, w12, #1            
        
    b       loop_2
.p2align 4
end_loop_2:  
            
    strb    w12, [x10]
    sub     x10, x10, #1           
    ldrb    w12, [x10]
        
    add     w12, w12, #2           
        
    strb    w12, [x9]
    mov     x0, #1                       
    mov     x1, x9
    mov     x2, #1
    mov     x16, #4
    svc     0
        
    sub     w12, w12, #12            
        
    strb    w12, [x9]
    mov     x0, #1                       
    mov     x1, x9
    mov     x2, #1
    mov     x16, #4
    svc     0
        
    strb     w12, [x10]                
    add      x10, x10, #1
    ldrb     w12, [x10]
        
    add     w12, w12, #6           
        
.p2align 4
loop_3:   
    cbz     w12, end_loop_3
            
    strb    w12, [x10]
    sub     x10, x10, #1           
    ldrb    w12, [x10]
        
    add     w12, w12, #9           
        
    strb     w12, [x10]                
    add      x10, x10, #1
    ldrb     w12, [x10]
        
    sub     w12, w12, #1            
        
    b       loop_3
.p2align 4
end_loop_3:  
            
    strb    w12, [x10]
    sub     x10, x10, #1           
    ldrb    w12, [x10]
        
    add     w12, w12, #1           
        
    strb    w12, [x9]
    mov     x0, #1                       
    mov     x1, x9
    mov     x2, #1
    mov     x16, #4
    svc     0
        
    strb    w12, [x10]
    sub     x10, x10, #1           
    ldrb    w12, [x10]
        
    strb    w12, [x9]
    mov     x0, #1                       
    mov     x1, x9
    mov     x2, #1
    mov     x16, #4
    svc     0
        
    add     w12, w12, #3           
        
    strb    w12, [x9]
    mov     x0, #1                       
    mov     x1, x9
    mov     x2, #1
    mov     x16, #4
    svc     0
        
    sub     w12, w12, #6            
        
    strb    w12, [x9]
    mov     x0, #1                       
    mov     x1, x9
    mov     x2, #1
    mov     x16, #4
    svc     0
        
    sub     w12, w12, #8            
        
    strb    w12, [x9]
    mov     x0, #1                       
    mov     x1, x9
    mov     x2, #1
    mov     x16, #4
    svc     0
        
    strb     w12, [x10]                
    add      x10, x10, #3
    ldrb     w12, [x10]
        
    add     w12, w12, #4           
        
.p2align 4
loop_4:   
    cbz     w12, end_loop_4
            
    strb    w12, [x10]
    sub     x10, x10, #1           
    ldrb    w12, [x10]
        
    add     w12, w12, #8           
        
    strb     w12, [x10]                
    add      x10, x10, #1
    ldrb     w12, [x10]
        
    sub     w12, w12, #1            
        
    b       loop_4
.p2align 4
end_loop_4:  
            
    strb    w12, [x10]
    sub     x10, x10, #1           
    ldrb    w12, [x10]
        
    add     w12, w12, #1           
        
    strb    w12, [x9]
    mov     x0, #1                       
    mov     x1, x9
    mov     x2, #1
    mov     x16, #4
    svc     0
        
    strb     w12, [x10]                
    add      x10, x10, #1
    ldrb     w12, [x10]
        
    add     w12, w12, #10           
        
    strb    w12, [x9]
    mov     x0, #1                       
    mov     x1, x9
    mov     x2, #1
    mov     x16, #4
    svc     0
        
.p2align 4                               
end:    
    mov     x0, #0
    mov     x16, #1
    svc     0
    