use serde_json::json;

pub fn phone() {
  // O tipo de `john` é `serde_json::Value`
  let john = json!({
    "name": "Neviim Jads",
    "age": 99,
    "phones": [
        "+55 1234567",
        "+55 7654321"
    ]
  });

  println!();
  println!("numero do primeiro telefone: {}", john["phones"][0]);

  // Converter para string o JSON e mostrar na saida
  println!("{}", john.to_string());
}