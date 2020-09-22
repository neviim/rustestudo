mod rotinas;

fn main() {

    rotinas::jsons::phone_modelo1();
    rotinas::jsons::phone_modelo2();

    match rotinas::cidade::print_an_address() {
        Ok(a) => a,
        _ => unreachable!(),
    };

}