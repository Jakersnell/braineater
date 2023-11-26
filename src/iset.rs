use std::process::Command;

pub struct M1;
impl M1 {
    pub const ENTRY_POINT: &'static str = "_main";
    pub const FILE_EX: &'static str = ".s";

    pub fn compile_to_bin_cmd(filename: &str) -> Vec<Command> {
        let mut cmd = Command::new("gcc");
        cmd.args(["-o", &filename, &(filename.to_owned() + Self::FILE_EX)]);
        vec![cmd]
    }

    pub fn init_header() -> String {
        format!(
            "
.section __DATA, __data
    .p2align 4
    MEM:
        .zero 8192
    
    .p2align 4
    BUF:
        .zero 256

.section    __TEXT,__text
.global {0}                             
                                        
.p2align 4
{0}:

    ; init buf ptr
    adrp    x9, BUF@PAGE
    add     x9, x9, BUF@PAGEOFF

    ; init mem ptr
    adrp    x10, MEM@PAGE
    add     x10, x10, MEM@PAGEOFF

    ; value register
    mov     w12, #0

        ",
            Self::ENTRY_POINT
        )
    }

    pub fn gen_inc_ptr(val: u32) -> String {
        format!(
            "
    strb     w12, [x10]                
    add      x10, x10, #{val}
    ldrb     w12, [x10]
        "
        )
    }

    pub fn gen_dec_ptr(val: u32) -> String {
        format!(
            "
    strb    w12, [x10]
    sub     x10, x10, #{val}           
    ldrb    w12, [x10]
        "
        )
    }

    pub fn gen_inc_val(val: u32) -> String {
        format!(
            "
    add     w12, w12, #{val}           
        "
        )
    }

    pub fn gen_dec_val(val: u32) -> String {
        format!(
            "
    sub     w12, w12, #{val}            
        "
        )
    }

    pub fn gen_lp_start(loop_label: usize) -> String {
        format!(
            "
.p2align 4
loop_{0}:   
    cbz     w12, end_loop_{0}
            ",
            loop_label
        )
    }

    pub fn gen_lp_end(loop_label: usize) -> String {
        format!(
            "
    b       loop_{0}
.p2align 4
end_loop_{0}:  
            ",
            loop_label
        )
    }

    pub fn gen_read() -> String {
        "
    mov     x1, x9                      
    mov     x0, #1
    mov     x2, #255
    mov     x16, #3
    svc     0
    ldrb    w12, [x9]
        "
        .to_string()
    }

    pub fn gen_write() -> String {
        "
    strb    w12, [x9]
    mov     x0, #1                       
    mov     x1, x9
    mov     x2, #1
    mov     x16, #4
    svc     0
        "
        .to_string()
    }

    pub fn init_footer() -> String {
        "
.p2align 4                               
end:    
    mov     x0, #0
    mov     x16, #1
    svc     0
    "
        .to_string()
    }
}
