static ACC: usize = 1; // 1mm/s /s

struct Race {
    time: usize,
    distance: usize,
}
impl Race {
    fn is_win(&self, button_time: usize) -> bool {
        let final_speed = ACC * button_time;
        let remaining_time = self.time - button_time;
        let distance_traveled = final_speed * remaining_time;
        if distance_traveled >= self.distance {
            true
        } else {
            false
        }
    }
}
fn main() {
    let times = [40, 82, 84, 92];
    let distances = [233, 1011, 1110, 1487];
    let races: Vec<Race> = times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| Race {
            time: *time,
            distance: *distance,
        })
        .collect();
    let mut wins: Vec<usize>= Vec::with_capacity(races.len());
    for race in races {
        let mut smallest = None;
        let mut biggest = None;
        for i in 0..=race.time {
            if smallest.is_none() && race.is_win(i) {
                smallest = Some(i);
            }
            if race.is_win(i) {
                biggest = Some(i);
            }
        }
        let smallest = smallest.unwrap();
        let biggest = biggest.unwrap();
        wins.push(biggest-smallest+1);
    }
    let product: usize = wins.iter().product();
    println!("Day 5 Part 1: {product}");

}
