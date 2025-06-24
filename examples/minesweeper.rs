pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return vec![];
    }

    let rows = minefield.len();
    let cols = minefield[0].len();
    let mut annotated_minefield: Vec<String> = Vec::with_capacity(rows);

    for r in 0..rows {
        let mut row_string = String::with_capacity(cols);
        for c in 0..cols {
            if minefield[r].as_bytes()[c] == b'*' {
                row_string.push('*');
                continue;
            }

            let mut mine_count = 0;
            for dr in -1..=1 {
                for dc in -1..=1 {
                    if dr == 0 && dc == 0 {
                        continue;
                    }

                    let nr = r as isize + dr;
                    let nc = c as isize + dc;

                    let in_row_range = nr >= 0 && nr < rows as isize;
                    let in_col_range = nc >= 0 && nc < cols as isize;
                    if in_row_range
                        && in_col_range
                        && minefield[nr as usize].as_bytes()[nc as usize] == b'*'
                    {
                        mine_count += 1;
                    }
                }
            }

            if mine_count > 0 {
                row_string.push_str(&mine_count.to_string());
            } else {
                row_string.push(' ');
            }
        }
        annotated_minefield.push(row_string);
    }

    annotated_minefield
}
#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn no_rows() {
        let input = &[];
        let expected: &[&str] = &[];
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    // #[ignore]
    fn no_columns() {
        let input = &[""];
        let expected = &[""];
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    // #[ignore]
    fn no_mines() {
        #[rustfmt::skip]
        let (input, expected) = (&[
            "   ",
            "   ",
            "   ",
        ], &[
            "   ",
            "   ",
            "   ",
        ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    // #[ignore]
    fn minefield_with_only_mines() {
        #[rustfmt::skip]
        let (input, expected) = (&[
            "***",
            "***",
            "***",
        ], &[
            "***",
            "***",
            "***",
        ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    // #[ignore]
    fn mine_surrounded_by_spaces() {
        #[rustfmt::skip]
        let (input, expected) = (&[
            "   ",
            " * ",
            "   ",
        ], &[
            "111",
            "1*1",
            "111",
        ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    // #[ignore]
    fn space_surrounded_by_mines() {
        #[rustfmt::skip]
        let (input, expected) = (&[
            "***",
            "* *",
            "***",
        ], &[
            "***",
            "*8*",
            "***",
        ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    // #[ignore]
    fn horizontal_line() {
        let input = &[" * * "];
        let expected = &["1*2*1"];
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    // #[ignore]
    fn horizontal_line_mines_at_edges() {
        let input = &["*   *"];
        let expected = &["*1 1*"];
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    // #[ignore]
    fn vertical_line() {
        #[rustfmt::skip]
        let (input, expected) = (&[
            " ",
            "*",
            " ",
            "*",
            " ",
        ], &[
            "1",
            "*",
            "2",
            "*",
            "1",
        ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    // #[ignore]
    fn vertical_line_mines_at_edges() {
        #[rustfmt::skip]
        let (input, expected) = (&[
            "*",
            " ",
            " ",
            " ",
            "*",
        ], &[
            "*",
            "1",
            " ",
            "1",
            "*",
        ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    // #[ignore]
    fn cross() {
        #[rustfmt::skip]
        let (input, expected) = (&[
            "  *  ",
            "  *  ",
            "*****",
            "  *  ",
            "  *  ",
        ], &[
            " 2*2 ",
            "25*52",
            "*****",
            "25*52",
            " 2*2 ",
        ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    // #[ignore]
    fn large_minefield() {
        #[rustfmt::skip]
        let (input, expected) = (&[
            " *  * ",
            "  *   ",
            "    * ",
            "   * *",
            " *  * ",
            "      ",
        ], &[
            "1*22*1",
            "12*322",
            " 123*2",
            "112*4*",
            "1*22*2",
            "111111",
        ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
}

fn main() {}
