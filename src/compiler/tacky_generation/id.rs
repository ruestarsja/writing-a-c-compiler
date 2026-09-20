pub(super) enum IDType {
    TemporaryVariableName,
    AndLabel,
    OrLabel,
    EndBooleanOperatorLabel,
    ElseLabel,
    EndIfLabel
}


pub(super) fn generate(
    label_id_type: IDType
) -> String {
    static mut VAR_COUNTER: u32 = 0;

    static mut AND_LABEL_COUNTER: u32 = 0;
    static mut OR_LABEL_COUNTER: u32 = 0;
    static mut END_BOOLOP_LABEL_COUNTER: u32 = 0;

    static mut ELSE_LABEL_COUNTER: u32 = 0;
    static mut END_IF_LABEL_COUNTER: u32 = 0;

    let count: u32;
    match label_id_type {
        IDType::TemporaryVariableName => {
            unsafe {
                count = VAR_COUNTER;
                VAR_COUNTER += 1;
            }
            format!("$tmp{}", count)
        },
        IDType::AndLabel => {
            unsafe {
                count = AND_LABEL_COUNTER;
                AND_LABEL_COUNTER += 1;
            }
            format!("and_false_{}", count)
        },
        IDType::OrLabel => {
            unsafe {
                count = OR_LABEL_COUNTER;
                OR_LABEL_COUNTER += 1;
            }
            format!("or_true_{}", count)
        },
        IDType::EndBooleanOperatorLabel => {
            unsafe {
                count = END_BOOLOP_LABEL_COUNTER;
                END_BOOLOP_LABEL_COUNTER += 1;
            }
            format!("end_boolop_{}", count)
        }
        IDType::ElseLabel => {
            unsafe {
                count = END_IF_LABEL_COUNTER;
                END_IF_LABEL_COUNTER += 1;
            }
            format!("else_{}", count)
        },
        IDType::EndIfLabel => {
            unsafe {
                count = ELSE_LABEL_COUNTER;
                ELSE_LABEL_COUNTER += 1;
            }
            format!("end_if_{}", count)
        }
    }
}