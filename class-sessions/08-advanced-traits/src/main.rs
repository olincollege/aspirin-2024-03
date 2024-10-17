use std::collections::LinkedList;

fn main() {
    let arr = LinkedList::from([1, 2, 3, 4, 5]);
    println!("{:?}", remove_evens(arr));
}

fn remove_evens<I>(arr: I) -> Vec<i32>
where
    I: IntoIterator<Item = i32>,
{
    arr.into_iter().filter(|x| x % 2 != 0).collect()
}
