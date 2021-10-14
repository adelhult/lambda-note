/// indents each line of the string by a given amount
pub fn indent(text: &str, level: usize) -> String {
    text
        .lines()
        .map(|line| format!("{:ident$}{msg}\n", "", ident=4*level, msg=line))
        .collect()
}