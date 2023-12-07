use std::{fs::read_to_string, str::FromStr, collections:: BTreeMap};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    A,
}

impl TryFrom<char> for Card {
    type Error = char;
    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            '2' => Ok(Card::Two),
            '3' => Ok(Card::Three),
            '4' => Ok(Card::Four),
            '5' => Ok(Card::Five),
            '6' => Ok(Card::Six),
            '7' => Ok(Card::Seven),
            '8' => Ok(Card::Eight),
            '9' => Ok(Card::Nine),
            'T' => Ok(Card::Ten),
            'J' => Ok(Card::Jack),
            'Q' => Ok(Card::Queen),
            'K' => Ok(Card::King),
            'A' => Ok(Card::A),
            _ => Err(value),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum HandType {
    High(Vec<Card>),
    Pair(Vec<Card>),
    TwoPair(Vec<Card>),
    Three(Vec<Card>),
    FullHouse(Vec<Card>),
    Four(Vec<Card>),
    Five(Vec<Card>),
}

impl FromStr for HandType {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hand: Vec<Card> = s.chars().map(|c| Card::try_from(c).unwrap()).collect();
        let mut counts: BTreeMap<Card, usize> = BTreeMap::new();
        for c in hand.iter() {
            if let Some(acc) = counts.get_mut(&c) {
                *acc += 1;
            } else {
                counts.insert(*c, 1);
            }
        }
        let mut hand_type: Vec<usize>= counts.into_iter().map(|x| x.1).collect();
        hand_type.sort();
        match &hand_type[..] {
            [5] => Ok(HandType::Five(hand)),
            [1,4] => Ok(HandType::Four(hand)),
            [2,3] => Ok(HandType::FullHouse(hand)),
            [1,1,3] => Ok(HandType::Three(hand)),
            [1,2,2] => Ok(HandType::TwoPair(hand)),
            [1,1,1,2] => Ok(HandType::Pair(hand)),
            _ => Ok(HandType::High(hand)),
        }
    }
}

#[derive(Debug)]
struct Hand {
    hand: HandType,
    bet: usize,
}

fn main() {
    let input = read_to_string("input.txt").expect("File 'input.txt' not found in cwd");
    let input: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.split(" ").collect())
        .collect();
    let mut hands: Vec<Hand> = input
        .into_iter()
        .map(|s|
             Hand { hand: HandType::from_str(s[0]).unwrap(), bet: str::parse(s[1]).unwrap()})
        .collect();
    hands.sort_by(|x,y| x.hand.cmp(&y.hand));
    let mut sum = 0;
    for (mut rank, hand) in hands.iter().enumerate() {
        rank += 1;
        sum += rank * hand.bet;
    }
    println!("Day 7 Part 1: {sum}");
}
