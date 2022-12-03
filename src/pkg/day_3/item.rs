use std::ops::Range;

#[derive(PartialEq, Clone)]
pub(crate) struct Item {
    code: char,
}

impl From<char> for Item {
    fn from(code: char) -> Self {
        return Item { code };
    }
}

impl Item {
    pub fn get_priority(&self) -> u32 {
        let ascii: u32 = self.code as u32;

        let lowercase_range: Range<u32> = 97..123;
        let uppercase_range: Range<u32> = 65..91;

        if lowercase_range.contains(&ascii) {
            return ascii - lowercase_range.start + 1;
        } else if uppercase_range.contains(&ascii) {
            return ascii - uppercase_range.start + 27;
        } else {
            panic!("The character=\"{}\" is invalid.", self.code);
        }
    }
}
