#[derive(Debug, Clone, Copy)]
struct Post<'a> {
    id: &'a str,
    name: &'a str,
    allocated: f32,
}

#[derive(Debug, Clone, Copy)]
struct Movement<'a> {
    amount: f32,
    payee: &'a str,
    timestamp: &'a str,
    post_id: &'a str,
}

#[derive(Debug)]
struct Budget<'a> {
    posts: Vec<Post<'a>>,
    movements: Vec<Movement<'a>>,
}

impl<'a> Budget<'a> {
    fn remaining_in_post(&'a self, post_id: &'a str) -> f32 {
        let deductions = self
            .movements
            .iter()
            .filter(|movement| movement.post_id == post_id)
            .fold(0.0f32, |accum, movement| accum + movement.amount);

        let allocated_in_post = self
            .posts
            .iter()
            .find(|post| post.id == post_id)
            .unwrap()
            .allocated;

        return allocated_in_post - deductions;
    }

    fn add_movement(&mut self, movement: Movement<'a>) {
        self.movements.push(movement);
    }

    fn print_movements(&self) {
        self.movements.iter().for_each(|movement| {
            let post = self.posts.iter().find(|post| post.id == movement.post_id);

            let post_name = match post {
                Some(post_ref) => post_ref.name,
                None => "Unknown",
            };

            println!(
                "[{:<22}] {:<15} {}: {}",
                movement.timestamp, movement.payee, post_name, movement.amount
            )
        })
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

    let mut test_data: Budget = Budget {
        movements: vec![test_movement.clone()],
        posts: vec![test_post.clone()],
    };

    println!("{:?}", &test_data);
    println!("{:?}", &test_data.remaining_in_post(&test_post.id));

    test_data.add_movement(Movement {
        amount: 25.20,
        payee: "Kiwi",
        post_id: "groceries",
        timestamp: "2022-10-31T16:40:40",
    });

    println!("{:?}", test_data.remaining_in_post(&test_post.id));
    test_data.print_movements();
}
