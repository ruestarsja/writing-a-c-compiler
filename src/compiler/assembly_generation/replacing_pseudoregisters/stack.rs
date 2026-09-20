static mut ABS_STACK_OFFSET: u32 = 0;


pub(super) fn get_new_offset() -> i32 {
    let offset: u32;
    unsafe {
        ABS_STACK_OFFSET += 4;
        offset = ABS_STACK_OFFSET;
    }

    -1 * offset.cast_signed()
}


pub(super) fn get_total_offset() -> u32 {
    unsafe { ABS_STACK_OFFSET }
}