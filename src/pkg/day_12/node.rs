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

    pub fn can_reach(&self, height: Height) -> bool {
        match height {
            Height::NodeHeight(other_h) => match self.height {
                Height::NodeHeight(h) => other_h <= h + 1,
                Height::Start => true,
                Height::End => false,
            },
            Height::Start => false,
            Height::End => match self.height {
                Height::NodeHeight(h) => h == 25,
                Height::Start => true,
                Height::End => false,
            },
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
