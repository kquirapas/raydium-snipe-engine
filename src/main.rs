// #![cfg(feature = "test-sbf")]

mod error;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn sanity_check() {
        assert_eq!(true, true);
    }
}
