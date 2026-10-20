pub(super) enum LoopType {
    While,
    DoWhile,
    For
}


pub(super) fn generate_loop_id(
    loop_type: LoopType
) -> String {
    static mut WHILE_LOOP_COUNTER: u32 = 0;
    static mut DO_WHILE_LOOP_COUNTER: u32 = 0;
    static mut FOR_LOOP_COUNTER: u32 = 0;

    let count: u32;
    match loop_type {
        LoopType::While => unsafe {
            count = WHILE_LOOP_COUNTER;
            WHILE_LOOP_COUNTER += 1;
        },
        LoopType::DoWhile => unsafe {
            count = DO_WHILE_LOOP_COUNTER;
            DO_WHILE_LOOP_COUNTER += 1;
        },
        LoopType::For => unsafe {
            count = FOR_LOOP_COUNTER;
            FOR_LOOP_COUNTER += 1;
        }
    }

    match loop_type {
        LoopType::While   => format!("while_{}",    count),
        LoopType::DoWhile => format!("do_while_{}", count),
        LoopType::For     => format!("for_{}",      count)
    }
}