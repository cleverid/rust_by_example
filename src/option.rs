#[cfg(test)]
mod tests {
    #[test]
    fn create() {
        let opt = Some(123);
        let value = opt.unwrap_or_else(|| 321);
        println!("{value}");
    }
}
