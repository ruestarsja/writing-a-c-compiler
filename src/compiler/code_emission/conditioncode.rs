use super::aast::ConditionCode;


pub(super) fn emit(
    aast_condition_code: ConditionCode
) -> String {
    match aast_condition_code {

        ConditionCode::E
            => "e".to_owned(),
        ConditionCode::NE
            => "ne".to_owned(),
        ConditionCode::L
            => "l".to_owned(),
        ConditionCode::LE
            => "le".to_owned(),
        ConditionCode::G
            => "g".to_owned(),
        ConditionCode::GE
            => "ge".to_owned()

    }
}