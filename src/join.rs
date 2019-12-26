use crate::classifier::Violation;

pub fn join_violations(violations: &Vec<Violation>) -> String {
    violations
        .iter()
        .map(|v| v.name().to_owned())
        .collect::<Vec<String>>()
        .join(", ")
}
