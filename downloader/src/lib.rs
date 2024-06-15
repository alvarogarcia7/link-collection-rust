pub mod tests {
    use reqwest::{header, Error};

    #[test]
    fn main() -> Result<(), Error> {
        let response = reqwest::blocking::Client::new();
        let response = response
            .get("http://0.0.0.0:8181/api/v1/reservations/1")
            .header(header::USER_AGENT, "My Rust Application")
            .send()?
            .text()?;

        println!("{}", response);

        Ok(())
    }
}
