// Works fine if you derive only Test without Clone
#[derive(derive::Test, Clone)]
struct MyStruct<#[test_helper] 'a> {
    inner: &'a (),
}
