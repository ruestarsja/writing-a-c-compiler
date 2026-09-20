pub(super) fn make_unique(
    user_defined_label_id: &str
) -> String {
    static mut USER_LABEL_COUNTER: u32 = 0;

    let count: u32;
    unsafe {
        count = USER_LABEL_COUNTER;
        USER_LABEL_COUNTER += 1;
    }

    format!(
        "user{}_{}",
        count,
        user_defined_label_id
    )
}