// const LETTERS: &[char] = letters();

fn letters() -> Vec<char> {
    ('a'..='z').chain('A'..='B').chain('0'..='9').collect()
}

pub struct CharIter {
    rng: rand::rngs::ThreadRng,
    chars: Vec<char>,
}

impl CharIter {
    fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
            chars: letters(),
        }
    }
}

impl Iterator for CharIter {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        use rand::Rng;
        let idx: usize = self.rng.gen_range(0..self.chars.len());
        let c = self.chars[idx];
        Some(c)
    }
}

trait WordLen {
    type Item;
    fn rand_len(&self) -> Self::Item;
}

trait NumericBound {
    fn min_value() -> Self;
    fn max_value() -> Self;
}

pub struct Word<H> {
    len: H,
}

// pub struct WordIter<'a, H> {
//     w: &'a Word<H>,
//     char_iter: CharIter,
// }

impl<H> Word<H> {
    fn new(r: H) -> Self {
        Word { len: r }
    }
}

// impl<'a, H> Iterator for WordIter<'a, H> where H: NumericBound + Copy {
//     type Item = String;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         let word_len = self.w.rand_len();
//         let char_vector: Vec<char> = self.char_iter.take(word_len).collect();
//         //let w: String = char_vector.iter().collect();
//         //println!("word {} {} {}", w, char_vector.len(), word_len);
//         None
//     }
// }

impl<Idx> WordLen for Word<std::ops::Range<Idx>>
where
    Idx: NumericBound + Copy + PartialOrd + rand::distributions::uniform::SampleUniform,
{
    type Item = Idx;
    fn rand_len(&self) -> Idx {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let random_number: Self::Item = rng.gen_range(self.len.start..=self.len.end);
        random_number
    }
}

impl<Idx> WordLen for Word<Idx>
where
    Idx: NumericBound + Copy,
{
    type Item = Idx;
    fn rand_len(&self) -> Self::Item {
        self.len
    }
}

pub fn word() {
    let word = Word::new(5u8..15);
    let word_len = word.rand_len();
    let char_iter = CharIter::new();
    let char_vector: Vec<char> = char_iter.take(word_len as usize).collect();
    let w: String = char_vector.iter().collect();
    println!("word {} {} {}", w, char_vector.len(), word_len);

    let word = Word::new(6u8);
    let word_len = word.rand_len();
    let char_iter = CharIter::new();
    let char_vector: Vec<char> = char_iter.take(word_len as usize).collect();
    let w: String = char_vector.iter().collect();
    println!("word {} {} {}", w, char_vector.len(), word_len);
}

impl NumericBound for u8 {
    fn min_value() -> Self {
        u8::MIN
    }

    fn max_value() -> Self {
        u8::MAX
    }
}

impl NumericBound for u16 {
    fn min_value() -> Self {
        u16::MIN
    }

    fn max_value() -> Self {
        u16::MAX
    }
}

impl NumericBound for u32 {
    fn min_value() -> Self {
        u32::MIN
    }

    fn max_value() -> Self {
        u32::MAX
    }
}

impl NumericBound for usize {
    fn min_value() -> Self {
        usize::MIN
    }

    fn max_value() -> Self {
        usize::MAX
    }
}

impl NumericBound for u64 {
    fn min_value() -> Self {
        u64::MIN
    }

    fn max_value() -> Self {
        u64::MAX
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_word() {
        super::word()
    }
}
