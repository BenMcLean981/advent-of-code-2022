use super::{shape::Shape, winner::Winner};

pub(crate) struct Game {
    my_play: Shape,
    winner: Winner,
}

impl Game {
    pub(crate) fn new_part_1(my_play: Shape, their_play: Shape) -> Self {
        return Self {
            my_play: my_play.clone(),
            winner: Self::get_winner(&my_play, &their_play),
        };
    }

    fn get_winner(my_play: &Shape, their_play: &Shape) -> Winner {
        match my_play {
            Shape::Rock => Self::get_winner_if_my_play_is_rock(their_play),
            Shape::Paper => Self::get_winner_if_my_play_is_paper(their_play),
            Shape::Scissors => Self::get_winner_if_my_play_is_scissors(their_play),
        }
    }

    fn get_winner_if_my_play_is_rock(their_play: &Shape) -> Winner {
        match their_play {
            Shape::Rock => Winner::Draw,
            Shape::Paper => Winner::Them,
            Shape::Scissors => Winner::Me,
        }
    }

    fn get_winner_if_my_play_is_paper(their_play: &Shape) -> Winner {
        match their_play {
            Shape::Rock => Winner::Me,
            Shape::Paper => Winner::Draw,
            Shape::Scissors => Winner::Them,
        }
    }

    fn get_winner_if_my_play_is_scissors(their_play: &Shape) -> Winner {
        match their_play {
            Shape::Rock => Winner::Them,
            Shape::Paper => Winner::Me,
            Shape::Scissors => Winner::Draw,
        }
    }

    pub(crate) fn new_part_2(their_play: Shape, winner: Winner) -> Self {
        return Self {
            my_play: Self::get_my_play(&their_play, &winner),
            winner: winner.clone(),
        };
    }

    fn get_my_play(their_play: &Shape, winner: &Winner) -> Shape {
        match winner {
            Winner::Me => Self::get_my_play_if_i_need_to_win(their_play),
            Winner::Them => Self::get_my_play_if_they_need_to_win(their_play),
            Winner::Draw => Self::get_my_play_if_i_need_a_draw(their_play),
        }
    }

    fn get_my_play_if_i_need_to_win(their_play: &Shape) -> Shape {
        match their_play {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }

    fn get_my_play_if_they_need_to_win(their_play: &Shape) -> Shape {
        match their_play {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }

    fn get_my_play_if_i_need_a_draw(their_play: &Shape) -> Shape {
        return their_play.clone();
    }

    pub(crate) fn get_score(&self) -> u32 {
        return self.get_score_for_shape() + self.get_score_for_winner();
    }

    fn get_score_for_shape(&self) -> u32 {
        match self.my_play {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn get_score_for_winner(&self) -> u32 {
        match self.winner {
            Winner::Me => 6,
            Winner::Them => 0,
            Winner::Draw => 3,
        }
    }
}
