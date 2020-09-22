mod inaccessible;
pub mod nested;

fn private_function() {
  println!("Acessado por: `my::private_function()`");
}

pub fn function() {
  println!("Acessado por: `my::function()`");
}

pub fn indirect_access() {
  println!("Acessado por: `my::indirect_access()`, que chamo");

  private_function();

  // acessa_inaccessible();
}

// fn acessa_inaccessible() {
//   println!("Acessado por: `inaccessible::private_function()`");

//   inaccessible::public_function();
// }