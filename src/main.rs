use my_parser::list_parser;

pub fn main() {
    println!("{:?}", list_parser::list("[1,1,2,3,5,8]"));
    assert_eq!(
        Ok(vec![1, 1, 2, 3, 5, 8]),
        list_parser::list("[1,1,2,3,5,8]")
    );
}
