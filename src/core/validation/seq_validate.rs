use regex::Regex;

pub fn evalidate_task_id(task_id:&str) -> bool {
    let task_id_pattern = Regex::new(r"\d{0,4}_\d+").unwrap();
    task_id_pattern.is_match(task_id)
}

#[cfg(test)]
mod test{
    use super::evalidate_task_id;

    #[test]
    pub fn test_task_id(){
        assert_eq!(evalidate_task_id("131_98321754"),true);
        assert_eq!(evalidate_task_id("2931487_94327549128"),true);
    }
}
