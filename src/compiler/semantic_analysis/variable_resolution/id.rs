pub(super) fn make_unique(
    user_defined_variable_id: &str
) -> String {
    static mut LOCAL_VARIABLE_COUNTER: u32 = 0;

    let count: u32;
    unsafe {
        count = LOCAL_VARIABLE_COUNTER;
        LOCAL_VARIABLE_COUNTER += 1;
    }
    
    format!(
        "$loc{}.{}",
        count,
        user_defined_variable_id
    )
}