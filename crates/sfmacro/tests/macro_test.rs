use sfmacro::addfields;

#[addfields(title="hello")]
struct User{}

#[test]
fn test_macro() {
    println!("hello");
    let _ = User{};
}