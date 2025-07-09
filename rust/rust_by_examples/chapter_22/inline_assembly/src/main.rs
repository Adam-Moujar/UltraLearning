// We can use raw assembly if needed, kinda like C.
// I mostly avoid it like the plague I'll be honest, I hadn't used it even when I coded in C. But
// its useful to know if I need it in the future, though I'll likely forget all about it
//
// Currently there are 4 supported architectures:
// x86 and x86-64
// ARM
// AArch64
// RISC-V
//
// Lets start with some basics

use std::arch::asm;

fn main() {
    let x: u64;
    unsafe {
        asm!("nop"); // No operation, it does literally nothing
        asm!("mov {}, 5", out(reg) x);
    }
    assert_eq!(x, 5);

    let i: u64 = 3;
    let o: u64;
    unsafe {
        asm!(
            "mov {0}, {1}", // move into {0}, out(reg) o, whatever is in {1}, in(reg) i,
            "add {0}, 5", // add into {0}, 5
            out(reg) o,
            in(reg) i,
        );
    }
    assert_eq!(o, 8);
    // we can refine the above assembly into
    use std::arch::asm;

    let mut x: u64 = 3;
    unsafe {
        asm!("add {0}, 5", inout(reg) x);
    }
    assert_eq!(x, 8);
    // or even instead of using x for both input and output, we can seperate them using =>
    let x: u64 = 3;
    let y: u64;
    unsafe {
        asm!("add {0}, 5", inout(reg) x => y);
    }
    assert_eq!(y, 8);

    // There are certain general purpose registers that x86 are available for general purpose use.
    // They are:
    // eax
    // ebx
    // ecx
    // edx
    // ebp
    // esi
    // edi
    //
    // Heres an example of using them
    //let cmd = 0xd1;
    //unsafe {
    //    asm!("out 0x64, eax", in("eax") cmd); // we output the contentds of cmd to port 0x64
    //}
    //segfaults idk why
    //i assume we cant access 0x64?

    //heres one using mul
    fn mul(a: u64, b: u64) -> u128 {
        let lo: u64;
        let hi: u64;

        unsafe {
            asm!(
                // The x86 mul instruction takes rax as an implicit input and writes
                // the 128-bit result of the multiplication to rax:rdx.
                "mul {}",
                in(reg) a, // theres one explicit operand a
                inlateout("rax") b => lo, // the next operand, b , is implicit, it will use
                // whatever is in register rax, so we populate rax with b and put the output in lo
                lateout("rdx") hi // the upper 64 bit of the multiplication will be stored in the
                // register rdx which again we store in hi
            );
        }

        ((hi as u128) << 64) + lo as u128
    }

    assert_eq!(mul(10, 24), 240);

    // sometimes assembly will modify memory/state that is not required
    // Either because the register is used as a temp, or because we modify state for the operation
    // and dont require it afterwards
    // We need to tell the compiler when this happens
    // for example:
    let mut name_buf = [0_u8; 12];
    unsafe {
        asm!(
            "push rbx",
            "cpuid",
            "mov [rdi], ebx",
            "mov [rdi + 4], edx",
            "mov [rdi + 8], ecx",
            "pop rbx",
            // We use a pointer to an array for storing the values to simplify
            // the Rust code at the cost of a couple more asm instructions
            // This is more explicit with how the asm works however, as opposed
            // to explicit register outputs such as `out("ecx") val`
            // The *pointer itself* is only an input even though it's written behind
            in("rdi") name_buf.as_mut_ptr(),
            // select cpuid 0, also specify eax as clobbered
            inout("eax") 0 => _,
            // cpuid clobbers these registers too
            out("ecx") _,
            out("edx") _,
        );
    }

    let name = core::str::from_utf8(&name_buf).unwrap();
    println!("CPU Manufacturer ID: {}", name);
    // Notice the inout("eax") in the assembly, at no point do we actually read or write eax,
    // this is because it happens during the cpuid (i think) and so we need to tell the compiler to
    // empty whatever is left in there using => _
}
