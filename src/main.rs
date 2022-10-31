#[derive(Debug)]
struct Post<'a> {
    id: &'a str,
    name: &'a str,
    allocated: f32,
}

#[derive(Debug)]
struct Movement<'a> {
    amount: f32,
    payee: &'a str,
    timestamp: &'a str,
    post_id: &'a str,
}

#[derive(Debug)]
struct Budget<'a> {
    posts: Vec<&'a Post<'a>>,
    movements: Vec<&'a Movement<'a>>,
}

impl Budget<'_> {
    fn money_left_in_post(self, post_id: &str) -> f32 {
        let deductions = self
            .movements
            .into_iter()
            .filter(|movement| movement.post_id == post_id)
            .fold(0.0f32, |accum, movement| accum + movement.amount);

        return self
            .posts
            .into_iter()
            .find(|post| post.id == post_id)
            .unwrap()
            .allocated
            - deductions;
    }
}

fn main() {
    let test_post = Post {
        id: "groceries",
        name: "Groceries 🍌",
        allocated: 3000.0,
    };

    let test_movement = Movement {
        amount: 236.90,
        payee: "REMA 1000",
        timestamp: "2022-10-31T16:35:40",
        post_id: &test_post.id,
    };

    let test_data: Budget = Budget {
        movements: vec![&test_movement],
        posts: vec![&test_post],
    };

    println!("{:?}", test_data);
    println!("{:?}", test_data.money_left_in_post(&test_post.id));
}
