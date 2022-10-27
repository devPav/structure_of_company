use std::{io, collections::HashMap};

fn main() {
    let mut struct_of_company = HashMap::new();
    loop {
        println!("Введите подразделение:");
        let mut department = String::new();
        io::stdin()
            .read_line(&mut department)
            .expect("Error i/o OS!");
        let department = String::from(department.trim());

        println!("Введите сотрудника:");
        let mut employ = String::new();
        io::stdin().read_line(&mut employ).expect("Error i/o OS!");
        let employ = String::from(employ.trim());

        let emploes_in_department = struct_of_company.entry(department).or_insert(Vec::new());

        emploes_in_department.push(employ);

        println!("Чтобы продолжить введите 'y', чтобы прервать введите любой символ:");
        let mut action = String::new();
        io::stdin().read_line(&mut action).expect("Error i/o OS!");
        match action.trim() {
            "y" | "Y" => continue,
            _ => break,
        }
    }
    println!("Структура предприятия: {:?}", struct_of_company);

    println!("Введите отдел:");
    let mut department = String::new();
    io::stdin()
        .read_line(&mut department)
        .expect("Error i/o OS!");
    let department = String::from(department.trim());
    match struct_of_company.get(&department) {
        Some(value) => println!("{:?}", value),
        None => (),
    }
}
