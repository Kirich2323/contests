// Kata's link: https://www.codewars.com/kata/57e3f79c9cb119374600046b

fn hello(name: &str) -> String {
    let mut c = name.chars();
    
    let name = match c.next() {
        None => "World".to_string(),
        Some(f) => f.to_uppercase()
                    .chain(c.map(|x| x.to_lowercase()
                        .collect::<String>()).collect::<String>().chars())
                    .collect()
    };
    format!("Hello, {name}!")
}