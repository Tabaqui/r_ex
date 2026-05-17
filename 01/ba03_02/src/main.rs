fn main() {
    let raw_args = std::env::args().skip(1).collect::<Vec<String>>();

    let f = !raw_args.is_empty();

    let ordered = get_sorted(raw_args);

    ordered.iter().for_each(|param| println!("{}", param));

    if f {
        println!();
    }
}

fn get_sorted(raw: Vec<String>) -> Vec<String> {
    if raw.is_empty() || raw.len() == 1 {
        return raw;
    }

    

    raw.iter().fold(vec![], |mut acc: Vec<String>, next_arg| {
        let count = acc
            .iter()
            .filter(|arg| {
                next_arg.gt(arg)
            })
            .count();

        if acc.is_empty() {
            acc.push((*next_arg).clone());
        } else {
            acc.insert(count, (*next_arg).clone());
        }

        acc
    })
}

#[test]
fn sort_empty() {
    let raw = vec![];

    let ordered = get_sorted(raw.clone());

    assert_eq!(raw.len(), ordered.len());
}

#[test]
fn sort_single() {
    let raw = vec![String::from("aA")];

    let ordered = get_sorted(raw.clone());

    assert_eq!(raw.len(), ordered.len());
    assert_eq!(raw, ordered);
}

#[test]
fn sort_ordered() {
    let raw_little = vec![String::from("a"), String::from("b")];
    let raw_big = vec![String::from("A"), String::from("b")];
    let raw_case_A = vec![String::from("A"), String::from("a")];
    let raw_case_B = vec![String::from("B"), String::from("a")];


    println!("L");
    let ordered_little = get_sorted(raw_little.clone());
    let ordered_big = get_sorted(raw_big.clone());
    let ordered_case_A = get_sorted(raw_case_A.clone());
    let ordered_case_B = get_sorted(raw_case_B.clone());


    assert_eq!(raw_little, ordered_little);
    assert_eq!(raw_big, ordered_big);
    assert_eq!(raw_case_A, ordered_case_A);
    assert_eq!(raw_case_B, ordered_case_B);

}

#[test]
fn sort_unordered() {
    let raw_little = vec![String::from("b"), String::from("a")];
    let raw_case = vec![String::from("a"), String::from("A")];

    let ordered_little = get_sorted(raw_little.clone());
    let ordered_case = get_sorted(raw_case.clone());

    assert_ne!(raw_little, ordered_little);
    assert_ne!(raw_case, ordered_case);
}
