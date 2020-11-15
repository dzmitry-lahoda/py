
/// reads line from stdin, trim empty end and start
/// # Examples
/// ```
/// assert_eq!(py::input(),"");
/// ```
/// #
pub fn input() -> String{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_owned()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
