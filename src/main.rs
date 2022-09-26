mod ex1;
use ex1::Invoice;

fn main() {
    let invoices: Vec<Invoice> = Invoice::random(1, 5);

    //ex1(&invoices);
    //ex2(&invoices);
    ex3(&invoices);
}

fn ex1(invoices: &Vec<Invoice>) {
    for invoice in invoices {
        let mut invoice_sum = 0.0;

        for position in &invoice.positions {
            let price = position.amount as f32 * position.price;
            invoice_sum += price;
        }

        println!("Sum: {:#?}", invoice_sum);
    }
}
fn ex2(invoices: &Vec<Invoice>) {
    invoices.iter().for_each(|invoice| {
        let invoice_sum = invoice
            .positions
            .iter()
            .fold(0.0, |prev, el| prev + el.amount as f32 * el.price);

        println!("Sum: {:#?}", &invoice_sum);
    })
}

fn ex3(invoices: &Vec<Invoice>) {
    invoices.iter().for_each(Invoice::print);
}
