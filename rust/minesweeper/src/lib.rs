pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let max_cols = if minefield.len() > 0 {
        minefield[0].len()
    } else {
        0
    };
    let mut annotated_minefield: Vec<Vec<i8>> = vec![vec![0; max_cols]; minefield.len()];

    minefield.iter().enumerate().for_each(|(i, row)| {
        row.bytes().enumerate().for_each(|(j, ch)| {
            if ch == b'*' {
                for x in i.saturating_sub(1)..=(i + 1).min(minefield.len() - 1) {
                    for y in j.saturating_sub(1)..=(j + 1).min(minefield[0].len() - 1) {
                        if minefield[x].chars().nth(y) != Some('*') {
                            annotated_minefield[x][y] += 1;
                        }
                    }
                }
                annotated_minefield[i][j] = -1;
            }
        });
    });

    annotated_minefield
        .iter()
        .map(|row| {
            row.iter()
                .map(|&count| match count {
                    -1 => '*',
                    0 => ' ',
                    _ => (b'0' + count as u8) as char,
                })
                .collect()
        })
        .collect()
}
