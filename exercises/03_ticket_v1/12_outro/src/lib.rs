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

pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32
}

impl Order {

   fn validate_product_name(product_name: String) -> String {
   
     if product_name.is_empty() {
       panic!("product_name cannot be empty!")
     }   
     if product_name.len() > 300 {
            panic!("product_name must be 300 bytes or less!")
     }
     product_name
   }

   fn validate_quantity (quantity: u32) -> u32 {
     if quantity < 1 {
       panic!("quantity must be > 0")
     }
     quantity
   }

   fn validate_unit_price(unit_price: u32) -> u32 {
     if unit_price < 1 {
       panic!("unit_price must be > 0")
     }     
     unit_price
   }
   
   pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Self {
      Order {
        product_name: Self::validate_product_name(product_name),
	quantity : Self::validate_quantity(quantity),
	unit_price : Self:: validate_unit_price(unit_price)
     }
   }

  pub fn product_name(&mut self) -> &String {
    &self.product_name
  }

  pub fn quantity(&mut self) -> &u32 {
    &self.quantity
  }

  pub fn unit_price(&mut self) -> &u32 {
    &self.unit_price
  }

  pub fn set_product_name(&mut self, value: String) {
    self.product_name = Self::validate_product_name(value)
  }

  pub fn set_quantity(&mut self, value: u32) {
    self.quantity = Self::validate_quantity(value)
  }

  pub fn set_unit_price(&mut self, value: u32) {
    self.unit_price = Self::validate_unit_price(value)
  }

  pub fn total(&mut self) -> u32 {
    return self.unit_price * self.quantity
  }
  
}
