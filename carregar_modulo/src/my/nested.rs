pub fn function() {
  println!("Acessado por: `my::nested::function()`");
}

#[allow(dead_code)]
fn private_function() {
  println!("Acessado por: `my::nested::private_function()`");
}
