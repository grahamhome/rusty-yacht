mod tests;

#[derive(Clone, Copy)]
pub enum Category {
    Ones = 1,
    Twos = 2,
    Threes = 3,
    Fours = 4,
    Fives = 5,
    Sixes = 6,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];
pub fn score(dice: Dice, category: Category) -> u8 {
    let dice_counts: Vec<u8> = dice
        .iter()
        .fold(vec![0, 0, 0, 0, 0, 0], |mut counts, &die| {
            counts[die as usize - 1] += 1;
            counts
        });
    match category {
        Category::Ones
        | Category::Twos
        | Category::Threes
        | Category::Fours
        | Category::Fives
        | Category::Sixes => dice.iter().filter(|&&d| d == category as u8).sum(),
        Category::FullHouse if dice_counts.contains(&3) && dice_counts.contains(&2) => {
            dice.iter().sum()
        }
        Category::FourOfAKind => dice_counts
            .iter()
            .enumerate()
            .map(|(i, &v)| if v >= 4 { (i as u8 + 1) * 4 } else { 0 })
            .sum(),
        Category::LittleStraight if dice_counts.iter().take(5).all(|v| v > &0) => 30,
        Category::BigStraight if dice_counts.iter().skip(1).all(|v| v > &0) => 30,
        Category::Choice => dice.iter().sum(),
        Category::Yacht if dice_counts.contains(&5) => 50,
        _ => 0,
    }
}
