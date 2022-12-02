#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Elf {
    total: i64,
}

impl Elf {
    fn new() -> Self {
        Elf { total: 0 }
    }

    fn add_cals(&mut self, calories: i64) {
        self.total += calories;
    }
}

fn main() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string("input.txt")?;
    let mut elf = Elf::new();
    let mut elves: Vec<Elf> = Vec::new();
    for line in input.split('\n') {
        if line.is_empty() {
            elves.push(elf);
            elf = Elf::new();
            continue;
        }
        if let Ok(calories) = line.parse::<i64>() {
            elf.add_cals(calories);
        }
    }
    elves.sort_by(|a, b| b.total.cmp(&a.total));
    println!(
        "The elf carrying the most calories is carrying {} calories",
        elves[0].total
    );

    let top_3: i64 = elves.iter().take(3).map(|elf| elf.total).sum();
    println!("The top 3 elves are totally carrying {} calories", top_3);

    Ok(())
}
