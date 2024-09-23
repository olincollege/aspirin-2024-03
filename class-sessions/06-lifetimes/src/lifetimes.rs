fn get_longer<'a, 'b>(first: &'a str, second: &'b str) -> &'a str
where
    'b: 'a,
{
    if first.len() > second.len() {
        first
    } else {
        second
    }
}

fn return_string_if_delimeter_in_string<'a>(string: &'a str, delimeter: &str) -> &'a str {
    if string.contains(delimeter) {
        string
    } else {
        ""
    }
}

fn use_delimeter() {
    let longer_lifetime = String::from("Helloooo");
    let result;
    {
        let delimeter = String::from("oo")
        result = return_string_if_delimeter_in_string(&longer_lifetime, &delimeter);
    }

    println!("{}", result);
}

fn use_longer() {
    let longer_lifetime = String::from("Hellosdfsdfhjsbhj");

    {
        let longer_word;
        {
            let shorter_lifetime = String::from("world!");
            longer_word = get_longer(&shorter_lifetime, &longer_lifetime);
            println!("longest: {}", longer_word);
        }
    }

    println!("{}", longer_lifetime);
}

fn find_in_string<'a, 'b>(sentence: &'a str, word: &'b str) -> Option<&'b str> {
    if let Some(_) = sentence.find(word) {
        Some(word)
    } else {
        None
    }
}

fn non_lexical_lifetime() {
    let longer_lifetime = String::from("Hello, world!");
    {
        let longest_word;
        let shorter_lifetime = String::from("hello");
        {
            longest_word = get_longer(&longer_lifetime, &shorter_lifetime);

        }
        println!("{}", longest_word);
    }
}