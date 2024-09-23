mod hashmap;
mod strings;
mod vectors;
use std::collections::HashMap;

fn main() {
    // println!("Hello, World!");
    // vectors_vs_arrays();
    // intro_to_iterators();
    // double_vec();
    // filter_vec();
    // iterate_over_strings();
    // intro_to_hashmaps();
    // get_mode();
}

fn vectors_vs_arrays() {
    let my_array = [0, 1, 2, 3];
    let my_vec = vec![0, 1, 2, 3];

    let idx = 2;
    // let idx = 4;
    let third = my_vec.get(idx);
    println!("{third:?}");
    let third = my_vec[idx];
    println!("{third}");
}

fn intro_to_iterators() {
    let mut rand_vec = Vec::new();

    for i in 0..4 {
        rand_vec.push(i);
    }

    // println!("{:?}", rand_vec);
    let mut iterator = rand_vec.into_iter();

    // println!("First: {:?}", iterator.next());
    // println!("Second: {:?}", iterator.next());
    // println!("Third: {:?}", iterator.next());
    // println!("Fourth: {:?}", iterator.next());
    // println!("Fifth: {:?}", iterator.next());
    // // What do you think will happen if .next() is called again?
    // println!("Sixth: {:?}", iterator.next());

    let mut next_val;
    loop {
        next_val = iterator.next();
        if next_val.is_none() {
            break;
        }

        // do something
        println!("val: {}", next_val.unwrap());
    }
}

fn double_vec() {
    let mut my_vec = vec![0, 1, 2, 32, 4, 3, 1, 2, 3, 2, 1, 1, 1, 2, 3];

    // base implementation
    // let mut double_vec = Vec::new();
    // for elem in my_vec.into_iter() {
    //     double_vec.push(elem * 2);
    // }

    // Functional programming
    // let double_vec: Vec<i32> = my_vec.into_iter().map(|elem| elem * 2).collect();

    // In Place
    // for elem in my_vec.iter_mut() {
    //     *elem *= 2;
    // }

    // In Place Functional
    my_vec.iter_mut().for_each(|x| *x *= 2);

    println!("{:?}", my_vec);
}

fn filter_vec() {
    let my_vec = vec![0, 1, 2, 32, 4, 3, 1, 2, 3, 2, 1, 1, 1, 2, 3];

    // base implementation
    // let mut filtered_vec = Vec::new();
    // for elem in my_vec.into_iter() {
    //     if elem % 2 == 0 {
    //         filtered_vec.push(elem * 2);
    //     }
    // }

    // Functional implementation
    let filtered_vec: Vec<i32> = my_vec.into_iter().filter(|x| x % 2 == 0).collect();
}

fn filter_and_double() {
    let my_vec = vec![0, 1, 2, 32, 4, 3, 1, 2, 3, 2, 1, 1, 1, 2, 3];

    // base implementation
    // let mut new_vec = Vec::new();
    // for elem in my_vec {
    //     if elem % 2 == 1 {
    //         new_vec.push(elem * 2);
    //     }
    // }

    // Functional
    // let new_vec: Vec<i32> = my_vec
    //     .into_iter()
    //     .filter(|x| x % 2 == 1)
    //     .map(|x| x * 2)
    //     .collect();
    let new_vec: Vec<u32> = my_vec
        .into_iter()
        .filter_map(|x| match x % 2 {
            0 => None,
            1 => Some(x * 2),
            _ => todo!(),
        })
        .collect();
}

fn iterate_over_strings() {
    let str_to_iterate_over = String::from("anmol");

    for char in str_to_iterate_over.chars() {
        println!("char: {}", char);
    }
}

fn intro_to_hashmaps() {
    // let mut map = HashMap::new();
    // map.insert("hi", "bye");
    // map.insert("you", "me");

    let map = HashMap::from([("hi", "bye"), ("you", "me")]);

    println!("{:?}", map);
}

fn get_mode() {
    let my_vec = vec![0, 1, 2, 32, 4, 3, 1, 2, 3, 2, 1, 1, 1, 2, 3];

    // let mut occurences = HashMap::new();
    // for elem in my_vec {
    //     // match occurences.contains_key(&elem) {
    //     //     true => *occurences.get_mut(&elem).unwrap() += 1,
    //     //     false => {
    //     //         occurences.insert(elem, 1);
    //     //         ()
    //     //     }
    //     // }
    //     occurences
    //         .entry(elem)
    //         .and_modify(|occ| *occ += 1)
    //         .or_insert(1);
    // }
    // println!("{:?}", occurences);

    let (max_val, _) = my_vec
        .into_iter()
        .fold(HashMap::new(), |mut mappings, new_elem| {
            mappings
                .entry(new_elem)
                .and_modify(|occ| *occ += 1)
                .or_insert(1);
            mappings
        })
        .into_iter()
        .max_by_key(|(_, val)| *val)
        .unwrap();

    // let mut max_val = 0;
    // let mut max_occurences = 0;
    // for (elem, occurences) in occurences.into_iter() {
    //     if occurences > max_occurences {
    //         max_val = elem;
    //         max_occurences = occurences;
    //     }
    // }
    // let (max_val, max_occurences) = occurences
    //     .into_iter()
    //     .max_by_key(|(key, val)| *val)
    //     .unwrap();

    println!("{:?}", max_val);
}
