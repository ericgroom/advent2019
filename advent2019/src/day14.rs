use crate::utils::read::read_list;
use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

type Quantity = usize;

#[derive(PartialEq, Eq, Debug, Clone, Default)]
struct ReactionItem {
    quantity: Quantity,
    id: String,
}

impl ReactionItem {
    fn new(quantity: Quantity, id: String) -> ReactionItem {
        ReactionItem {
            quantity: quantity,
            id: id,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Reaction {
    inputs: Vec<ReactionItem>,
    output: ReactionItem,
}

impl FromStr for Reaction {
    type Err = std::num::ParseIntError;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = string.split(" => ").collect();
        let input_str = parts[0];
        let output_str = parts[1];

        let mut inputs_result = Vec::new();

        for element in input_str.split(", ") {
            let elem_parts: Vec<_> = element.split(' ').collect();

            let quantity = elem_parts[0].parse()?;
            let id = String::from(elem_parts[1]);
            inputs_result.push(ReactionItem::new(quantity, id))
        }

        let elem_parts: Vec<_> = output_str.split(' ').collect();
        let quantity = elem_parts[0].parse()?;
        let id = String::from(elem_parts[1]);
        let output = ReactionItem::new(quantity, id);

        Ok(Reaction {
            inputs: inputs_result,
            output: output,
        })
    }
}

fn read_input(input: &str) -> Vec<Reaction> {
    read_list(input, "\n")
}

fn construct_reaction_tree(reactions: Vec<Reaction>) -> HashMap<String, Reaction> {
    let mut result = HashMap::new();
    for reaction in reactions {
        let output_id = reaction.output.id.clone();
        result.insert(output_id, reaction);
    }
    result
}

pub fn construct<'a, 'b>(
    element: &'a str,
    quantity: Quantity,
    all_reactions: &'a HashMap<String, Reaction>,
    materials: &'b mut HashMap<&'a str, Quantity>,
) -> usize {
    let mut stack = VecDeque::new();
    stack.push_front((element, quantity));
    let mut ores_consumed = 0;
    while !stack.is_empty() {
        let (element, mut quantity) = stack.pop_front().unwrap();
        let reaction = &all_reactions[element];

        let already_constructed_quanitity = materials.entry(element).or_default();
        let mut amount_extra = 0;
        if quantity <= *already_constructed_quanitity {
            amount_extra = *already_constructed_quanitity - quantity;
            quantity = 0;
            *already_constructed_quanitity = amount_extra;
        } else {
            quantity -= *already_constructed_quanitity;
            *already_constructed_quanitity = 0;
        }
        if quantity <= 0 {
            continue;
        }
        let reactions_needed = {
            let mut reaction_count = 0;
            while reaction_count * reaction.output.quantity < quantity {
                reaction_count += 1;
            }
            reaction_count
        };
        for input in reaction.inputs.iter() {
            if input.id == "ORE" {
                let ore_to_consume = input.quantity * reactions_needed;
                ores_consumed += ore_to_consume;
            } else {
                stack.push_front((&input.id, input.quantity * reactions_needed));
            }
        }
        amount_extra += reaction.output.quantity * reactions_needed - quantity;
        materials.insert(element, amount_extra);
    }
    return ores_consumed;
}

fn get_test_input() -> Vec<Reaction> {
    read_input(include_str!("day14_input.txt"))
}

pub fn find_ore_cost() -> usize {
    find_fuel_cost_in_ore(get_test_input())
}

pub fn find_fuel_cost_in_ore(reactions: Vec<Reaction>) -> usize {
    let map = construct_reaction_tree(reactions);
    let ores_consumed = construct("FUEL", 1, &map, &mut HashMap::new());
    ores_consumed
}

fn max_quantity_of_fuel(ore_count: i64, reactions: HashMap<String, Reaction>) -> i64 {
    let cost_of_one = construct("FUEL", 1, &reactions, &mut HashMap::new());
    let mut left = cost_of_one as i64;
    let mut right = ore_count / cost_of_one as i64 * 2;
    let mut fuel_count = 0;
    let mut cache: HashMap<i64, (i64, HashMap<&str, usize>)> = HashMap::new();
    while right >= left {
        let mid = left + (right - left) / 2;
        let (start_fuel, (cost, leftover)) = {
            let mut max_key = 0;
            for key in cache.keys() {
                if *key > max_key && *key <= mid {
                    max_key = *key;
                }
            }
            if let Some((ore_cost, materials)) = cache.get(&max_key) {
                (max_key, (*ore_cost, materials.to_owned()))
            } else {
                (0, (0, HashMap::new()))
            }
        };
        let mut materials: HashMap<&str, usize> = leftover;
        materials.insert("FUEL", start_fuel as usize);
        let ores_consumed =
            construct("FUEL", mid as usize, &reactions, &mut materials) as i64 + cost;
        cache.insert(mid, (ores_consumed, materials));
        if ores_consumed > ore_count {
            right = mid - 1;
        } else {
            left = mid + 1;
            fuel_count = mid;
        }
    }
    fuel_count
}

pub fn find_fuel_producible_from_1_trillion_ore() -> i64 {
    let input = get_test_input();
    let map = construct_reaction_tree(input);
    max_quantity_of_fuel(1_000_000_000_000, map)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reaction_from_str() {
        {
            let reaction_str = "44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL";
            let reaction = Reaction::from_str(reaction_str).unwrap();
            let expected = Reaction {
                inputs: vec![
                    ReactionItem::new(44, "XJWVT".into()),
                    ReactionItem::new(5, "KHKGT".into()),
                    ReactionItem::new(1, "QDVJ".into()),
                    ReactionItem::new(29, "NZVS".into()),
                    ReactionItem::new(9, "GPVTF".into()),
                    ReactionItem::new(48, "HKGWZ".into()),
                ],
                output: ReactionItem::new(1, "FUEL".into()),
            };
            assert_eq!(reaction, expected);
        }
        {
            let reaction_str = "44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL";
            let reaction = Reaction::from_str(reaction_str).unwrap();
            let expected = Reaction {
                inputs: vec![
                    ReactionItem::new(44, "XJWVT".into()),
                    ReactionItem::new(5, "KHKGT".into()),
                    ReactionItem::new(1, "QDVJ".into()),
                    ReactionItem::new(29, "NZVS".into()),
                    ReactionItem::new(9, "GPVTF".into()),
                    ReactionItem::new(48, "HKGWZ".into()),
                ],
                output: ReactionItem::new(1, "FUEL".into()),
            };
            assert_eq!(reaction, expected);
        }
    }

    #[test]
    fn test_construct_reaction_tree() {
        {
            let input = "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";
            let reactions = read_input(input);
            let tree = construct_reaction_tree(reactions);
            assert_eq!(tree.len(), 6);

            assert_eq!(
                tree["FUEL"],
                Reaction {
                    inputs: vec![
                        ReactionItem::new(7, "A".into()),
                        ReactionItem::new(1, "E".into())
                    ],
                    output: ReactionItem::new(1, "FUEL".into()),
                }
            );

            assert_eq!(
                tree["A"],
                Reaction {
                    inputs: vec![ReactionItem::new(10, "ORE".into())],
                    output: ReactionItem::new(10, "A".into()),
                }
            );
        }
        {
            let input = "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";
            let reactions = read_input(input);
            let tree = construct_reaction_tree(reactions);

            assert_eq!(
                tree["AB"],
                Reaction {
                    inputs: vec![
                        ReactionItem::new(3, "A".into()),
                        ReactionItem::new(4, "B".into())
                    ],
                    output: ReactionItem::new(1, "AB".into()),
                }
            );

            assert_eq!(
                tree["A"],
                Reaction {
                    inputs: vec![ReactionItem::new(9, "ORE".into())],
                    output: ReactionItem::new(2, "A".into()),
                }
            );
        }
    }

    #[test]
    fn test_find_fuel_cost_in_ore() {
        {
            let input = "1 ORE => 2 A
1 A => 1 B
1 A, 1 B => 1 FUEL";
            let reactions = read_input(input);
            assert_eq!(find_fuel_cost_in_ore(reactions), 1);
        }
        {
            let input = "10 ORE => 5 A
2 A => 3 B
7 B => 1 C
7 B, 1 C => 1 FUEL";
            let reactions = read_input(input);
            assert_eq!(find_fuel_cost_in_ore(reactions), 20);
        }
        {
            let input = "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";
            let reactions = read_input(input);
            assert_eq!(find_fuel_cost_in_ore(reactions), 31);
        }
        {
            let input = "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";
            let reactions = read_input(input);
            assert_eq!(find_fuel_cost_in_ore(reactions), 165);
        }
        {
            let input = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
            let reactions = read_input(input);
            assert_eq!(find_fuel_cost_in_ore(reactions), 13312);
        }
        {
            let input = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF";
            let reactions = read_input(input);
            assert_eq!(find_fuel_cost_in_ore(reactions), 180697);
        }
        {
            let input = "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";
            let reactions = read_input(input);
            assert_eq!(find_fuel_cost_in_ore(reactions), 2210736);
        }
    }

    #[test]
    fn test_correct_answer_part_1() {
        assert_eq!(find_ore_cost(), 857266);
    }

    #[test]
    fn test_max_quantity_of_fuel() {
        {
            let input = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
            let map = construct_reaction_tree(read_input(input));
            assert_eq!(max_quantity_of_fuel(1_000_000_000_000, map), 82892753);
        }
        {
            let input = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF";
            let map = construct_reaction_tree(read_input(input));
            assert_eq!(max_quantity_of_fuel(1_000_000_000_000, map), 5586022);
        }
        {
            let input = "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";
            let map = construct_reaction_tree(read_input(input));
            assert_eq!(max_quantity_of_fuel(1_000_000_000_000, map), 460664);
        }
    }

    #[test]
    fn test_correct_answer_part_2() {
        assert_eq!(find_fuel_producible_from_1_trillion_ore(), 2144702);
    }
}
