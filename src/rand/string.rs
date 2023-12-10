// const LETTERS: &[char] = letters();

use rand::Rng;

fn letters() {
    let chars = ('a'..='z').chain('A'..='B').chain('0'..='9');
}

pub struct CharIter {
    rng: rand::rngs::ThreadRng,
}

impl CharIter {
    fn new() -> Self {
        Self {
            rng: rand::thread_rng()
        }
    }

    fn iter(self) -> Self {
        CharIter::new()
    }
}

impl Iterator for CharIter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let t = Some(self.rng.gen());
        t
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

impl<H> Word<H> {
    fn new(r: H) -> Self {
        Word {
            len: r
        }
    }
}

impl<Idx> WordLen for Word<std::ops::Range<Idx>>
    where
        Idx: NumericBound
        + PartialOrd
        + rand::distributions::uniform::SampleUniform,
{
    type Item = Idx;
    fn rand_len(&self) -> Idx {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let random_number: Self::Item =
            rng.gen_range(Self::Item::min_value()..=Self::Item::max_value());
        random_number
    }
}

impl<Idx> WordLen for Word<Idx> where Idx: NumericBound + Copy {
    type Item = Idx;
    fn rand_len(&self) -> Self::Item {
        self.len
    }
}

pub fn word() {
    let word = Word::new(1u8..9);
    let word_len = word.rand_len();
    let char_iter = CharIter::new();
    let char_vector: Vec<u8> = char_iter.iter().take(word_len as usize).collect();
    let w: String = unsafe { String::from_utf8_unchecked(char_vector) };
    println!("word {}", w);

    let word = Word::new(1u8);
    let l = word.rand_len();
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