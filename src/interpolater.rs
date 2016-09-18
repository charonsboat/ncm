pub fn interpolate(string: &str, args: &[(&str, &str)]) -> String {
    let mut text = String::from(string);

    for arg in args {
        text = text.replace(arg.0, arg.1);
    }

    return text;
}
