// estruturas
struct Triangulo {
  base: f64,
  altura: f64
}

// metodos
impl Figura for Triangulo {

  fn area(&self) -> f64 {
      (self.base * self.altura) / 2.0  // Como se deve fazers
  }
}

// novo metodo
impl Triangulo {
  fn set_base(&mut self, base: f64) {
      self.base = base;
  }
}

// tratos
trait Figura {
  fn area(&self) -> f64;  // Que fazer
}

/*
fn main() {

    let mut triangulo = Triangulo {
        base: 10.0,
        altura: 20.0
    };

    triangulo.base = triangulo.base * 10.0;
    
    println!("base: {}, altura: {}", triangulo.base, triangulo.altura );
}
*/