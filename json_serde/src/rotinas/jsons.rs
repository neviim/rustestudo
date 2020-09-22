use serde_json::json;

pub fn phone_modelo1() {
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

fn random_phone() -> String {
  // Not too random.
  "1234567".to_owned()
}

pub fn phone_modelo2() {

  let nome_completo = "Neviim Jads";
  let idade_ultimo_ano = 42;
  
  // O tipo de `john` é `serde_json::Value`
  let john = json!({
      "name": nome_completo,
      "age": idade_ultimo_ano + 1,
      "phones": [
          format!("+55 {}", random_phone())
      ]
  });

  println!();
  println!("{}", john.to_string());

}