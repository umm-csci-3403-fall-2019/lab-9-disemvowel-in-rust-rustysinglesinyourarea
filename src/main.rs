fn main() {
    println!("Hello, world!");
    let s = String::from("Hello, world");
    let s_disemvowel = disemvowel(&s);

    println!("s was '{}', and without vowels is '{}'.", s, s_disemvowel);
}

fn disemvowel(word: &str) -> String {
    let v = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

    let char_vec: Vec<char> = word.chars().collect();

    let mut _disemvoweled_vec = String::from("");

    for chara in char_vec {
        if !v.contains(&chara) {
            _disemvoweled_vec.push(chara);
        }
    }

    _disemvoweled_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world() {
        let input = "Hello, world!";
        let expected = "Hll, wrld!";

        assert_eq!(
            expected,
            disemvowel(input)
        );
    }

    #[test]
    fn empty() {
        assert_eq!("", disemvowel(""));
    }

    #[test]
    fn no_vowels() {
        assert_eq!("pqrst", disemvowel("pqrst"));
    }

    #[test]
    fn all_vowels() {
        assert_eq!("", disemvowel("aeiouAEIOUOIEAuoiea"));
    }

    #[test]
    fn morris_minnesota() {
        assert_eq!("Mrrs, Mnnst", disemvowel("Morris, Minnesota"));
    }

    #[test]
    fn handle_punctuation() {
        assert_eq!("n (nxplnd) lphnt!", 
            disemvowel("An (Unexplained) Elephant!"));
    }

    #[test]
    fn handle_unicode() {
        assert_eq!("Sm hrglyphs: 𒐁	𒐌	𒐥	𒑳",
            disemvowel("Some hieroglyphs: 𒐁	𒐌	𒐥	𒑳"));
        assert_eq!("Sm Lnr B: 	𐂀	𐂚	𐃃	𐃺",
            disemvowel("Some Linear B: 	𐂀	𐂚	𐃃	𐃺"));
        assert_eq!(" lttl Phncn: 𐤀	𐤈	𐤔	𐤕",
            disemvowel("A little Phoenician: 𐤀	𐤈	𐤔	𐤕"));
        assert_eq!("W cn hndl mj s wll! 🤣😃👍",
            disemvowel("We can handle emoji as well! 🤣😃👍"))
    }
}