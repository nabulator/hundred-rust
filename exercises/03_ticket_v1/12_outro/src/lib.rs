// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.
//

pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32
}

fn check_p(p: String) -> String{
    if p.is_empty() || p.len() > 300 {
        panic!("!oh no");
    }
    p
}
fn check_q(q: u32) -> u32 {
    if q <= 0 {
        panic!("ohno!");
    }
    q
}
fn check_u(u: u32) -> u32 {
    if u <= 0 {
        panic!("!ohno");
    }
    u
}


impl Order {
    pub fn new(p:String, q:u32, u:u32) -> Order {
        Order {
            product_name: check_p(p),
            quantity: check_q(q),
            unit_price: check_u(u)
        }
    }
    pub fn total(&mut self) -> u32 {
        self.quantity * self.unit_price
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }
    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }
    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }


    // setters

    pub fn set_product_name(&mut self, new_p: String) {
        self.product_name = new_p;
    }
    pub fn set_quantity(&mut self, new_q: u32) {
        self.quantity = new_q;
    }
    pub fn set_unit_price(&mut self, new_u: u32) {
        self.unit_price = new_u;
    }

}
