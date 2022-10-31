#[derive(Debug)]
struct Post<'a> {
    id: &'a str,
    name: &'a str,
    allocated: f32,
}

#[derive(Debug)]
struct YearMonth(i16, i8);

#[derive(Debug)]
struct Budget<'a> {
    posts: Vec<Post<'a>>,
    date: &'a YearMonth,
}

#[derive(Debug)]
struct Movement<'a> {
    amount: f32,
    payee: &'a str,
    timestamp: &'a str,
    budget_id: &'a str,
}

#[derive(Debug)]
struct State<'a> {
    budgets: Vec<Budget<'a>>,
    movements: Vec<Budget<'a>>,
}

fn main() {
    let test_data: State = State {
        budgets: vec![Budget {
            posts: vec![],
            date: &YearMonth(2022, 10),
        }],
        movements: vec![],
    };

    println!("{:?}", test_data);
}
