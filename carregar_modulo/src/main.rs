// Esta declaração irá procurar por um arquivo chamado `my.rs` ou` my/mod.rs` 
// e irá insira seu conteúdo dentro de um módulo chamado `my` neste escopo.

mod my;

fn function() {
  println!("Acessado por: `function()`");
}

fn main() {
  println!();

  function();

  my::function();

  my::indirect_access();

  my::nested::function();

}


