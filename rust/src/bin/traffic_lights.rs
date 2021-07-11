use std::collections::{BTreeMap, BTreeSet};
use std::io::*;
use std::usize;

fn add_unlit_length(length_tree: &mut BTreeMap<usize, usize>, length: usize) {
    let value = length_tree.get_mut(&length);

    if let Some(inner) = value {
        *inner += 1;
    } else {
        length_tree.insert(length, 1);
    }
}

fn remove_unlit_length(length_tree: &mut BTreeMap<usize, usize>, length: usize) {
    let value = length_tree.get_mut(&length).unwrap();
    *value -= 1;

    if *value == 0 {
        length_tree.remove(&length);
    }
}

fn find_unlit_range(light_set: &BTreeSet<usize>, length: usize) -> (usize, usize) {
    use std::ops::Bound::*;

    let mut before = light_set.range((Unbounded, Excluded(length)));
    let mut after = light_set.range((Excluded(length), Unbounded));

    // unwrap will always work because lights will be within street bounds
    // according to input constraints
    (
        before.next_back().unwrap().clone(),
        after.next().unwrap().clone(),
    )
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut input = input.lines();
    let first_line: Vec<usize> = input
        .next()
        .unwrap()
        .split(' ')
        .into_iter()
        .map(|val| val.parse().unwrap())
        .collect();
    let street_lights_pos: Vec<usize> = input
        .next()
        .unwrap()
        .split(' ')
        .into_iter()
        .map(|val| val.parse().unwrap())
        .collect();
    let street_length = first_line[0];

    let mut light_set: BTreeSet<usize> = BTreeSet::new();
    light_set.insert(0);
    light_set.insert(street_length);

    let mut length_tree: BTreeMap<usize, usize> = BTreeMap::new();
    length_tree.insert(street_length, 1);

    for new_light_pos in street_lights_pos.iter() {
        // get the range of unlit portion new light will split
        let (left_boundary, right_boundary) = find_unlit_range(&light_set, *new_light_pos);

        // add new light to current set of lights
        light_set.insert(*new_light_pos);

        let unlit_length = right_boundary - left_boundary;
        let new_left_unlit_stretch = new_light_pos - left_boundary;
        let new_right_unlit_stretch = right_boundary - new_light_pos;

        // delete the previous unlit stretch
        remove_unlit_length(&mut length_tree, unlit_length);

        // add the two new unlit stretches created after the new light was added
        add_unlit_length(&mut length_tree, new_left_unlit_stretch);
        add_unlit_length(&mut length_tree, new_right_unlit_stretch);

        // print current max unlit distance
        let max_unlit = length_tree.iter().next_back().unwrap();
        print!("{} ", max_unlit.0);
    }
}
