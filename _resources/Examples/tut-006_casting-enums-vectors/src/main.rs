use std::mem;

fn main() {
    
    // ----- CASTING WITH AS -----
    // You can convert to different types in multiple ways
    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;
    // Cast using as unsigned int 32
    let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);
    println!("int3_u32={}, size of {} bytes", int3_u32, mem::size_of_val(&int3_u32));
}
