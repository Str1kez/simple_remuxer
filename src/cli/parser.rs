pub fn parse_args<T: Iterator<Item = String>>(
    mut args: T,
) -> Result<(String, String), &'static str> {
    args.next();
    let Some(input) = args.next() else {
        return Err("Couldn't get input path from args");
    };
    let Some(output) = args.next() else {
        return Err("Couldn't get output path from args");
    };
    Ok((input, output))
}
