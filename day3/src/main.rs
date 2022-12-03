use array_tool::vec::Intersect;

#[derive(Debug, PartialEq, Clone)]
struct Rucksack<'a> {
    cpt1: Compartment<'a>,
    cpt2: Compartment<'a>,
}

impl<'a> Rucksack<'a> {
    fn find_common_item(&self) -> Vec<&SupplyItem<'a>> {
        let mut common: Vec<&SupplyItem<'a>> = self
            .cpt1
            .items
            .iter()
            .filter_map(|itm| self.cpt2.items.iter().find(|&i| i == itm))
            .collect();
        common.dedup();
        common
    }

    fn merge_compartments(&self) -> Vec<&SupplyItem<'a>> {
        self.cpt1
            .items
            .iter()
            .chain(self.cpt2.items.iter())
            .collect()
    }
}
impl<'a> From<&'a str> for Rucksack<'a> {
    fn from(from: &'a str) -> Rucksack<'a> {
        let (cpt1, cpt2) = from.split_at(from.len() / 2);
        Rucksack {
            cpt1: Compartment::from(cpt1),
            cpt2: Compartment::from(cpt2),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Compartment<'a> {
    items: Vec<SupplyItem<'a>>,
}

impl<'a> From<&'a str> for Compartment<'a> {
    fn from(from: &'a str) -> Compartment<'a> {
        Compartment {
            items: from
                .split("")
                .into_iter()
                .filter(|c| c != &"")
                .map(SupplyItem::from)
                .collect(),
        }
    }
}
#[derive(Debug, PartialEq, Copy, Clone)]
struct SupplyItem<'a> {
    itemtype: Itemtype<'a>,
    priority: Priority,
}

const PRIOLIST: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
impl<'a> From<&'a str> for SupplyItem<'a> {
    fn from(from: &'a str) -> SupplyItem<'a> {
        let Some(prio) = PRIOLIST.find(from) else {
            panic!("invalid item");
        };
        SupplyItem {
            itemtype: Itemtype(from),
            priority: Priority(prio + 1),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Itemtype<'a>(&'a str);

#[derive(Debug, PartialEq, Copy, Clone)]
struct Priority(usize);

impl std::ops::Deref for Priority {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A group of three bags
#[derive(Debug, PartialEq)]
struct Group<'a> {
    bags: Vec<&'a Rucksack<'a>>,
}

impl<'a> Group<'a> {
    fn get_bag_contents_merged(&self) -> Vec<Vec<&SupplyItem<'a>>> {
        self.bags
            .clone()
            .into_iter()
            .map(|bag| bag.merge_compartments())
            .collect()
    }

    fn get_common_item(&self) -> Vec<&SupplyItem<'a>> {
        let merged = self.get_bag_contents_merged();
        let mut intersect = Vec::new();
        for bag in merged {
            if intersect.is_empty() {
                intersect = bag;
            } else {
                intersect = intersect.intersect(bag);
            }
        }
        intersect
    }
}

fn main() {
    // Find the item type that appears in both compartments of each rucksack. What is the sum of the priorities of those item types?
    let sum_priorities: usize = include_str!("../input.txt")
        .lines()
        .map(Rucksack::from)
        .map(|sack| {
            sack.find_common_item()
                .iter()
                .map(|item| *item.priority)
                .sum::<usize>()
        })
        .sum();

    println!("Sum of all priorities: {}", sum_priorities);

    let sacks: Vec<Rucksack> = include_str!("../input.txt")
        .lines()
        .map(Rucksack::from)
        .collect();

    let groups: Vec<Group> = sacks
        .chunks(3)
        .map(|sacks| Group {
            bags: vec![&sacks[0], &sacks[1], &sacks[2]],
        })
        .collect();

    let groups_score: usize = groups
        .iter()
        .map(|group| {
            group
                .get_common_item()
                .iter()
                .map(|item| *item.priority)
                .sum::<usize>()
        })
        .sum();

    println!("Sum of priorities in part two: {}", groups_score);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn try_parse_supplyitem_p() {
        let char = "p";
        let itm = SupplyItem::from(char); //.unwrap();
        assert_eq!(Itemtype("p"), itm.itemtype);
        assert_eq!(Priority(16), itm.priority);
    }

    #[test]
    fn try_parse_supplyitem_upper_l() {
        let char = "L";
        let itm = SupplyItem::from(char); //.unwrap();
        assert_eq!(Itemtype("L"), itm.itemtype);
        assert_eq!(Priority(38), itm.priority);
    }
    #[test]
    fn try_parse_supplyitem_upper_p() {
        let char = "P";
        let itm = SupplyItem::from(char); //.unwrap();
        assert_eq!(Itemtype("P"), itm.itemtype);
        assert_eq!(Priority(42), itm.priority);
    }

    #[test]
    fn try_parse_supplyitem_v() {
        let char = "v";
        let itm = SupplyItem::from(char); //.unwrap();
        assert_eq!(Itemtype("v"), itm.itemtype);
        assert_eq!(Priority(22), itm.priority);
    }

    #[test]
    fn try_parse_supplyitem_t() {
        let char = "t";
        let itm = SupplyItem::from(char); //.unwrap();
        assert_eq!(Itemtype("t"), itm.itemtype);
        assert_eq!(Priority(20), itm.priority);
    }

    #[test]
    fn try_parse_supplyitem_s() {
        let char = "s";
        let itm = SupplyItem::from(char); //.unwrap();
        assert_eq!(Itemtype("s"), itm.itemtype);
        assert_eq!(Priority(19), itm.priority);
    }

    #[test]
    fn compartment_parse_1() {
        let raw = "p";
        let cpt = Compartment::from(raw);
        assert_eq!(
            Compartment {
                items: vec![SupplyItem {
                    itemtype: Itemtype("p"),
                    priority: Priority(16)
                }]
            },
            cpt
        );
    }

    #[test]
    fn compartment_parse_2() {
        let raw = "psL";
        let cpt = Compartment::from(raw);
        assert_eq!(
            Compartment {
                items: vec![
                    SupplyItem {
                        itemtype: Itemtype("p"),
                        priority: Priority(16)
                    },
                    SupplyItem {
                        itemtype: Itemtype("s"),
                        priority: Priority(19)
                    },
                    SupplyItem {
                        itemtype: Itemtype("L"),
                        priority: Priority(38)
                    }
                ]
            },
            cpt
        );
    }
    #[test]
    fn find_common_1() {
        let raw = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";
        let rucksack = Rucksack::from(raw);
        let common = rucksack.find_common_item();

        assert_eq!(
            common,
            vec![&SupplyItem {
                itemtype: Itemtype("L"),
                priority: Priority(38)
            }]
        );

        assert_eq!(
            38,
            rucksack
                .find_common_item()
                .iter()
                .map(|item| *item.priority)
                .sum::<usize>()
        );
    }

    #[test]
    fn find_common_2() {
        let raw = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let rucksack = Rucksack::from(raw);
        let common = rucksack.find_common_item();
        assert_eq!(
            common,
            vec![&SupplyItem {
                itemtype: Itemtype("p"),
                priority: Priority(16)
            }]
        )
    }

    #[test]
    fn find_common_3() {
        let raw = "PmmdzqPrVvPwwTWBwg";
        let rucksack = Rucksack::from(raw);
        let common = rucksack.find_common_item();
        assert_eq!(
            common,
            vec![&SupplyItem {
                itemtype: Itemtype("P"),
                priority: Priority(42)
            }]
        )
    }

    #[test]
    fn rucksack_parse_1() {
        let raw = "PmmdzqPrVvPwwTWBwg";
        let rucksack = Rucksack::from(raw);

        let eq = Rucksack {
            /// PmmdzqPrV
            cpt1: Compartment {
                items: vec![
                    SupplyItem {
                        itemtype: Itemtype("P"),
                        priority: Priority(42),
                    },
                    SupplyItem {
                        itemtype: Itemtype("m"),
                        priority: Priority(13),
                    },
                    SupplyItem {
                        itemtype: Itemtype("m"),
                        priority: Priority(13),
                    },
                    SupplyItem {
                        itemtype: Itemtype("d"),
                        priority: Priority(4),
                    },
                    SupplyItem {
                        itemtype: Itemtype("z"),
                        priority: Priority(26),
                    },
                    SupplyItem {
                        itemtype: Itemtype("q"),
                        priority: Priority(17),
                    },
                    SupplyItem {
                        itemtype: Itemtype("P"),
                        priority: Priority(42),
                    },
                    SupplyItem {
                        itemtype: Itemtype("r"),
                        priority: Priority(18),
                    },
                    SupplyItem {
                        itemtype: Itemtype("V"),
                        priority: Priority(48),
                    },
                ],
            },
            /// vPwwTWBwg
            cpt2: Compartment {
                items: vec![
                    SupplyItem {
                        itemtype: Itemtype("v"),
                        priority: Priority(22),
                    },
                    SupplyItem {
                        itemtype: Itemtype("P"),
                        priority: Priority(42),
                    },
                    SupplyItem {
                        itemtype: Itemtype("w"),
                        priority: Priority(23),
                    },
                    SupplyItem {
                        itemtype: Itemtype("w"),
                        priority: Priority(23),
                    },
                    SupplyItem {
                        itemtype: Itemtype("T"),
                        priority: Priority(46),
                    },
                    SupplyItem {
                        itemtype: Itemtype("W"),
                        priority: Priority(49),
                    },
                    SupplyItem {
                        itemtype: Itemtype("B"),
                        priority: Priority(28),
                    },
                    SupplyItem {
                        itemtype: Itemtype("w"),
                        priority: Priority(23),
                    },
                    SupplyItem {
                        itemtype: Itemtype("g"),
                        priority: Priority(7),
                    },
                ],
            },
        };
        assert_eq!(eq, rucksack);
    }
}
