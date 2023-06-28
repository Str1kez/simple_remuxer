pub fn parse_args<T: Iterator<Item = String>>(
    mut args: T,
) -> Result<(String, String), &'static str> {
    args.next();
    let input = match args.next() {
        Some(path) => path,
        None => return Err("Couldn't get input path from args"),
    };
    let output = match args.next() {
        Some(path) => path,
        None => return Err("Couldn't get output path from args"),
    };
    Ok((input, output))
}
