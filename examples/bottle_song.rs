pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut song = String::new();
    for i in ((start_bottles - take_down + 1)..=start_bottles).rev() {
        song.push_str(&verse(i));
        if i > (start_bottles - take_down + 1) {
            song.push('\n');
        }
    }
    song
}

fn verse(n: u32) -> String {
    let next_bottles = n - 1;

    format!(
        "{n} hanging on the wall,\n\
         {n} hanging on the wall,\n\
         And if one green bottle should accidentally fall,\n\
         There'll be {next_bottles} hanging on the wall.\n",
        n = number_to_string(n),
        next_bottles = number_to_string(next_bottles).to_lowercase(),
    )
}

fn number_to_string(n: u32) -> String {
    match n {
        0 => "No green bottles".to_string(),
        1 => "One green bottle".to_string(),
        2 => "Two green bottles".to_string(),
        3 => "Three green bottles".to_string(),
        4 => "Four green bottles".to_string(),
        5 => "Five green bottles".to_string(),
        6 => "Six green bottles".to_string(),
        7 => "Seven green bottles".to_string(),
        8 => "Eight green bottles".to_string(),
        9 => "Nine green bottles".to_string(),
        10 => "Ten green bottles".to_string(),
        _ => unreachable!(),
    }
}
fn main() {}

#[cfg(test)]
mod test {
    use crate::recite;

    #[test]
    fn first_generic_verse() {
        assert_eq!(
            recite(10, 1).trim(),
            concat!(
                "Ten green bottles hanging on the wall,\n",
                "Ten green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be nine green bottles hanging on the wall.",
            )
        );
    }
    #[test]
    // #[ignore]
    fn last_generic_verse() {
        assert_eq!(
            recite(3, 1).trim(),
            concat!(
                "Three green bottles hanging on the wall,\n",
                "Three green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be two green bottles hanging on the wall.",
            )
        );
    }
    #[test]
    // #[ignore]
    fn verse_with_2_bottles() {
        assert_eq!(
            recite(2, 1).trim(),
            concat!(
                "Two green bottles hanging on the wall,\n",
                "Two green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be one green bottle hanging on the wall.",
            )
        );
    }
    #[test]
    // #[ignore]
    fn verse_with_1_bottle() {
        assert_eq!(
            recite(1, 1).trim(),
            concat!(
                "One green bottle hanging on the wall,\n",
                "One green bottle hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be no green bottles hanging on the wall.",
            )
        );
    }
    #[test]
    // #[ignore]
    fn first_two_verses() {
        assert_eq!(
            recite(10, 2).trim(),
            concat!(
                "Ten green bottles hanging on the wall,\n",
                "Ten green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be nine green bottles hanging on the wall.\n",
                "\n",
                "Nine green bottles hanging on the wall,\n",
                "Nine green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be eight green bottles hanging on the wall.",
            )
        );
    }
    #[test]
    // #[ignore]
    fn last_three_verses() {
        assert_eq!(
            recite(3, 3).trim(),
            concat!(
                "Three green bottles hanging on the wall,\n",
                "Three green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be two green bottles hanging on the wall.\n",
                "\n",
                "Two green bottles hanging on the wall,\n",
                "Two green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be one green bottle hanging on the wall.\n",
                "\n",
                "One green bottle hanging on the wall,\n",
                "One green bottle hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be no green bottles hanging on the wall.",
            )
        );
    }
    #[test]
    // #[ignore]
    fn all_verses() {
        assert_eq!(
            recite(10, 10).trim(),
            concat!(
                "Ten green bottles hanging on the wall,\n",
                "Ten green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be nine green bottles hanging on the wall.\n",
                "\n",
                "Nine green bottles hanging on the wall,\n",
                "Nine green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be eight green bottles hanging on the wall.\n",
                "\n",
                "Eight green bottles hanging on the wall,\n",
                "Eight green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be seven green bottles hanging on the wall.\n",
                "\n",
                "Seven green bottles hanging on the wall,\n",
                "Seven green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be six green bottles hanging on the wall.\n",
                "\n",
                "Six green bottles hanging on the wall,\n",
                "Six green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be five green bottles hanging on the wall.\n",
                "\n",
                "Five green bottles hanging on the wall,\n",
                "Five green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be four green bottles hanging on the wall.\n",
                "\n",
                "Four green bottles hanging on the wall,\n",
                "Four green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be three green bottles hanging on the wall.\n",
                "\n",
                "Three green bottles hanging on the wall,\n",
                "Three green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be two green bottles hanging on the wall.\n",
                "\n",
                "Two green bottles hanging on the wall,\n",
                "Two green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be one green bottle hanging on the wall.\n",
                "\n",
                "One green bottle hanging on the wall,\n",
                "One green bottle hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be no green bottles hanging on the wall.",
            )
        );
    }
}
