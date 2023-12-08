use std::{collections::BTreeMap, fs::read_to_string, str::FromStr};

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
        let mut hand_type: Vec<usize> = counts.into_iter().map(|x| x.1).collect();
        hand_type.sort();
        match &hand_type[..] {
            [5] => Ok(HandType::Five(hand)),
            [1, 4] => Ok(HandType::Four(hand)),
            [2, 3] => Ok(HandType::FullHouse(hand)),
            [1, 1, 3] => Ok(HandType::Three(hand)),
            [1, 2, 2] => Ok(HandType::TwoPair(hand)),
            [1, 1, 1, 2] => Ok(HandType::Pair(hand)),
            _ => Ok(HandType::High(hand)),
        }
    }
}

#[derive(Debug)]
struct Hand {
    hand: HandType,
    bet: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Card2 {
    Jack,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    A,
}

impl TryFrom<char> for Card2 {
    type Error = char;
    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            'T' => Ok(Self::Ten),
            'J' => Ok(Self::Jack),
            'Q' => Ok(Self::Queen),
            'K' => Ok(Self::King),
            'A' => Ok(Self::A),
            _ => Err(value),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum HandType2 {
    High(Vec<Card2>),
    Pair(Vec<Card2>),
    TwoPair(Vec<Card2>),
    Three(Vec<Card2>),
    FullHouse(Vec<Card2>),
    Four(Vec<Card2>),
    Five(Vec<Card2>),
}

impl FromStr for HandType2 {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hand: Vec<Card2> = s.chars().map(|c| Card2::try_from(c).unwrap()).collect();
        let mut counts: BTreeMap<Card2, usize> = BTreeMap::new();
        for c in hand.iter() {
            if let Some(acc) = counts.get_mut(&c) {
                *acc += 1;
            } else {
                counts.insert(*c, 1);
            }
        }
        let mut hand_type: Vec<usize> = counts.into_iter().map(|x| x.1).collect();
        hand_type.sort();
        let num_jacks: usize = hand
            .iter()
            .map(|x| match x {
                Card2::Jack => 1,
                _ => 0,
            })
            .sum();
        match &hand_type[..] {
            [5] => Ok(Self::Five(hand)),
            [1, 4] => {
                if num_jacks == 1 || num_jacks == 4 {
                    Ok(Self::Five(hand))
                } else {
                    Ok(Self::Four(hand))
                }
            }
            [2, 3] => {
                if num_jacks == 2 || num_jacks == 3 {
                    Ok(Self::Five(hand))
                } else {
                    Ok(Self::FullHouse(hand))
                }
            }
            [1, 1, 3] => {
                if num_jacks == 1 || num_jacks == 3 {
                    Ok(Self::Four(hand))
                } else {
                    Ok(Self::Three(hand))
                }
            }
            [1, 2, 2] => {
                if num_jacks == 1 {
                    Ok(Self::FullHouse(hand))
                } else if num_jacks == 2 {
                    Ok(Self::Four(hand))
                } else {
                    Ok(Self::TwoPair(hand))
                }
            }
            [1, 1, 1, 2] => {
                if num_jacks == 1 || num_jacks == 2 {
                    Ok(Self::Three(hand))
                } else {
                    Ok(Self::Pair(hand))
                }
            }
            x => {
                assert_eq!(x, [1,1,1,1,1]);
                if num_jacks == 1 {
                    Ok(Self::Pair(hand))
                } else {
                    Ok(Self::High(hand))
                }
            }
        }
    }
}
#[derive(Debug)]
struct Hand2 {
    hand: HandType2,
    bet: usize,
}

fn main() {
    let input = read_to_string("input.txt").expect("File 'input.txt' not found in cwd");
    let input: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.split(" ").collect())
        .collect();
    let mut hands: Vec<Hand> = input
        .iter()
        .map(|s| Hand {
            hand: HandType::from_str(s[0]).unwrap(),
            bet: str::parse(s[1]).unwrap(),
        })
        .collect();
    hands.sort_by(|x, y| x.hand.cmp(&y.hand));
    let mut sum = 0;
    for (rank, hand) in hands.iter().enumerate() {
        sum += (rank + 1) * hand.bet;
    }
    println!("Day 7 Part 1: {sum}");
    let mut hands: Vec<Hand2> = input
        .iter()
        .map(|s| Hand2 {
            hand: HandType2::from_str(s[0]).unwrap(),
            bet: str::parse(s[1]).unwrap(),
        })
        .collect();
    hands.sort_by(|x, y| x.hand.cmp(&y.hand));
    let mut sum = 0;
    for (rank, hand) in hands.iter().enumerate() {
        sum += (rank + 1) * hand.bet;
    }
    println!("Day 7 Part 2: {sum}");
}
