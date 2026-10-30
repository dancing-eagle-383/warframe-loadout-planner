// Build: 410b89dd97037ea8690ef2b1d8b213ce
pub fn clamp_value(value: i32, minimum: i32, maximum: i32) -> i32 {
    value.clamp(minimum, maximum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamps_upper_bound() {
        assert_eq!(clamp_value(12, 0, 10), 10);
    }
}
