use fake::{
    faker::{lorem::en, name::raw::Name},
    locales::{EN, ZH_TW},
    Dummy, Fake, Faker,
};

#[derive(Debug, Dummy)]
struct Foo {
    #[dummy(faker = "1000..2000")]
    _order_id: usize,
    _customer: String,
    _paid: bool,
}

#[derive(Debug, Dummy)]
struct Bar<T> {
    _field: Vec<T>,
}

// ref: https://github.com/cksac/fake-rs?tab=readme-ov-file#usage
fn main() {
    // type derived Dummy
    let f: Foo = Faker.fake();
    println!("Foo: {f:?}");

    let b: Bar<Foo> = Faker.fake();
    println!("Bar: {b:?}");

    // using `Faker` to generate default fake value of given type
    let tuple = Faker.fake::<(u8, u32, f32)>();
    println!("tuple: {tuple:?}");
    println!("String: {:?}", Faker.fake::<String>());

    // type U can used to generate fake value T, if `T: Dummy<U>`
    println!("String(range): {:?}", (8..20).fake::<String>());
    println!("u32(range): {:?}", (8..20).fake::<u32>());

    let name: String = Name(EN).fake();
    println!("name(EN): {name:?}");

    let name: String = Name(ZH_TW).fake();
    println!("name(ZH_TW): {name:?}");

    // using convenient function without providing locale
    let words: Vec<String> = en::Words(3..5).fake();
    println!("words: {words:?}");

    // using a tuple config list to generate a vector with a length range and a specific faker for the element
    let name_vec: Vec<String> = (Name(EN), 3..5).fake();
    println!("name vec(tuple config): {name_vec:?}");

    // using macro as an alternative method for the tuple config list
    let name_vec: Vec<String> = fake::vec![String as Name(EN); 3..5];
    println!("name vec(macro config): {name_vec:?}");

    // using macro to generate nested collection
    let name_vec = fake::vec![String as Name(EN); 4, 3..5, 2];
    println!("random nested vec: {name_vec:?}");
}
