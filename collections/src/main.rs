use std::collections::HashMap;
// Given a list of integers return 
// mean - average value
// median - when sorted the value in middle position
// and mode - value that occurs most often
fn vector_example(list: Vec<i32>) -> (i32, i32, i32) {
    // let sum = list.iter().sum();
    let mut sum = 0;
    for &value in list.iter() {
        sum += value;
    }
    let mean = sum / (list.len() as i32);
    // ==============
    let mut hashmap: HashMap <i32, i32> = HashMap::new();
    for &value in list.iter() {
        let count = hashmap.entry(value).or_insert(0);
        *count += 1;
    }

    let mut max_value: i32 = 0;
    let mut occurencies: i32 = 0;
    for (&key, &value) in hashmap.iter() {
        if value > occurencies {
            max_value = key;
            occurencies = value;
        }
    }
    //===========
    let mut vector = list;
    vector.sort();
    let median = vector.get(vector.len() / 2).expect("No value found");
    // =========
    (mean, *median, max_value)
}

// pig latin
// apple => apple-hay
// first => irst-fay
fn pig_latin(str: &str) -> String { 
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let char_vector: Vec<char> = str.chars().collect();

    if char_vector.is_empty() { 
        return String::from("")
    } else {
        let first = char_vector[0];
        if vowels.contains(&first) {
            let value = String::from(str) + "-hay";
            return value
        } else {
            let mut chars = str.chars();
            chars.next();
            let rest = String::from(chars.as_str());
            let first_string = first.to_string().to_owned();
            let dash = "-";
            let ay = "ay";
            return rest + dash + &first_string + ay
        }
    }
}

//Using a hash map and vectors, create a text interface to allow a user to add 
//employee names to a department in a company. 
//For example, “Add Sally to Engineering” or “Add Amir to Sales.” 
//Then let the user retrieve a list of all people in a department or 
//all people in the company by department, sorted alphabetically.
type Company = HashMap<String, Vec<String>>;
fn add_employee(company: &mut Company, text: &str) {
    let words: Vec<&str> = text.split(" ").collect();
    if words.len() < 4 {
        return;
    }
    let department = words.last().expect("Value is expected");
    let name = words[1..(words.len()-2)].join(" ");

    let department_staff = 
    company.entry(department.to_string()).or_insert(Vec::new());
    department_staff.push(name);
}

fn main() {
    let vector: Vec<i32> = vec![4, 5, 5, 6, 10, 1, 13];
    let (mean, median, max) = vector_example(vector);
    println!("Mean: {}, Median: {}, Max: {}", mean, median, max);

    println!("PIG LATIN");
    println!("{}", pig_latin("apple"));
    println!("{}", pig_latin("first"));
    println!("{}", pig_latin("ebay"));

    let mut company: Company = HashMap::new();
    add_employee(&mut company, "Add Sally to Engineering");
    add_employee(&mut company, "Add Amir to Sales");
    add_employee(&mut company, "Add Amir to Engineering");
    add_employee(&mut company, "Add Sally to Engineering");
    print!("{:#?}", company);
}
