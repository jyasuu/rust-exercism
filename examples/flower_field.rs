pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() {
        return vec![];
    }

    let num_rows = garden.len();
    let num_cols = garden[0].len();

    if num_cols == 0 {
        return vec![String::new(); num_rows];
    }

    let mut annotated_garden: Vec<Vec<char>> = vec![vec![' '; num_cols]; num_rows];

    for (r, row) in garden.iter().enumerate() {
        for (c, cell) in row.chars().enumerate() {
            if cell == '*' {
                annotated_garden[r][c] = '*';
            } else {
                let mut flower_count = 0;
                for dr in -1..=1 {
                    for dc in -1..=1 {
                        if dr == 0 && dc == 0 {
                            continue; // Skip the cell itself
                        }

                        let nr = r as isize + dr;
                        let nc = c as isize + dc;

                        // Check bounds
                        let in_range =
                            nr >= 0 && nr < num_rows as isize && nc >= 0 && nc < num_cols as isize;

                        if in_range {
                            let is_star = garden[nr as usize].chars().nth(nc as usize) == Some('*');
                            if is_star {
                                flower_count += 1;
                            }
                        }
                    }
                }
                if flower_count > 0 {
                    annotated_garden[r][c] = std::char::from_digit(flower_count, 10).unwrap();
                }
            }
        }
    }

    annotated_garden
        .into_iter()
        .map(|row| row.into_iter().collect())
        .collect()
}

fn main() {}

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
    fn no_flowers() {
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
    fn garden_full_of_flowers() {
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
    fn flower_surrounded_by_spaces() {
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
    fn space_surrounded_by_flowers() {
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
    fn horizontal_line_flowers_at_edges() {
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
    fn vertical_line_flowers_at_edges() {
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
    fn large_garden() {
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
