pub fn escape(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            '+' => '-',
            '/' => '_',
            _ => c,
        })
        .filter(|&c| c != '=')
        .collect()
}

pub fn unescape(input: &str) -> String {
    let m = input.len() % 4;
    let size = if m == 0 { 0 } else { 4 - m };
    let padding = vec!['='; size];

    input
        .chars()
        .chain(padding)
        .map(|c| match c {
            '-' => '+',
            '_' => '/',
            _ => c,
        })
        .collect()
}
