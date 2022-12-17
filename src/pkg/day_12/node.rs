#[derive(PartialEq, Clone, Copy, Eq, Hash)]
pub(crate) struct Node {
    pub height: Height,
    pub row: usize,
    pub col: usize,
}

const ASCII_FOR_LOWERCASE_A: usize = 97;

impl Node {
    pub fn new(height: char, row: usize, col: usize) -> Self {
        let height = Height::from(height);

        return Node { height, row, col };
    }

    pub fn can_reach_from(&self, from: Height) -> bool {
        match from {
            Height::NodeHeight(from_h) => match self.height {
                Height::NodeHeight(h) => h <= from_h + 1,
                Height::Start => false,
                Height::End => from_h == 25,
            },
            Height::Start => true,
            Height::End => false,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Eq, Hash)]
pub(crate) enum Height {
    NodeHeight(usize),
    Start,
    End,
}

impl From<char> for Height {
    fn from(c: char) -> Self {
        match c {
            'S' => Height::Start,
            'E' => Height::End,
            c => {
                let ascii = c as usize;
                let height = ascii - ASCII_FOR_LOWERCASE_A;
                return Height::NodeHeight(height);
            }
        }
    }
}
