use rand::prelude::*;

#[derive(Debug)]
pub struct Invoice {
    pub receipient: String,
    pub positions: Vec<InvoicePosition>,
}
#[derive(Debug)]
pub struct InvoicePosition {
    pub article_name: String,
    pub amount: i32,
    pub price: f32,
}

impl InvoicePosition {
    pub fn random(amount: i32) -> Vec<InvoicePosition> {
        (0..amount)
            .map(|_| -> InvoicePosition {
                InvoicePosition {
                    article_name: String::from(rand::thread_rng().gen_range(0..10).to_string()),
                    amount: rand::thread_rng().gen_range(0..10),
                    price: rand::thread_rng().gen_range(1.0..8.0),
                }
            })
            .collect()
    }
}

impl Invoice {
    pub fn map<'a, F, R>(&'a self, func: F) -> Vec<R>
    where
        F: Fn(&InvoicePosition) -> R,
    {
        self.positions.iter().map(&func).collect::<Vec<R>>()
    }

    pub fn sum(&self) -> f32 {
        return self
            .positions
            .iter()
            .fold(0.0, |prev: f32, el: &InvoicePosition| {
                prev + (el.price * el.amount as f32)
            });
    }

    pub fn random(amount: i32, amount_positions: i32) -> Vec<Self> {
        (0..amount)
            .map(|_| Invoice {
                receipient: String::from(rand::thread_rng().gen_range(0..10).to_string()),
                positions: InvoicePosition::random(amount_positions),
            })
            .collect()
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        println!("Receipient: {}", &self.receipient);
        self.print_positions();
    }

    #[allow(dead_code)]
    pub fn print_positions(&self) {
        println!("Article \t Amount \t Price \t\t Sum");
        println!("--------------------------------------------------------------------------");
        self.map(|f| {
            println!(
                "{} \t\t {} \t\t {} \t {}",
                &f.article_name,
                &f.amount,
                &f.price,
                &f.price * f.amount as f32
            );
        });
        println!("--------------------------------------------------------------------------");
        println!("To pay \t \t \t \t \t {}", self.sum());
    }
}
