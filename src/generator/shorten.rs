use core::fmt;

use super::name_generator::{GeneratedName, NameGeneratorTrait};
use url::Url;

pub trait NamesRepository {
    fn store_name(&self, original: &Url, generated: &GeneratedName) -> Result<(), String>;
    fn name_exists(&self, name: &GeneratedName) -> Result<bool, String>;
    fn retrieve_original_name(&self, name: &GeneratedName) -> Result<String, String>;
}

pub struct OutputLink(pub String);

impl fmt::Display for OutputLink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub trait Shortener {
    fn shorten_name(
        &self,
        name: &str,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Result<OutputLink, String>;
}

pub struct LinkShortener<T, B>
where
    T: NamesRepository,
    B: NameGeneratorTrait,
{
    base_url: String,
    names_repo: T,
    generator: B,
}
impl<T, B> LinkShortener<T, B>
where
    T: NamesRepository,
    B: NameGeneratorTrait,
{
    pub fn new(base_url: String, names_repo: T, generator: B) -> Self {
        Self {
            base_url,
            names_repo,
            generator,
        }
    }

    fn validate_input(&self, input_link: &str) -> Result<Url, String> {
        let url = Url::parse(input_link);
        url.map_err(|_| "Invalid URL".to_string())
    }

    fn to_output_link(&self, generated_name: &GeneratedName) -> OutputLink {
        OutputLink(self.base_url.clone() + "/" + &generated_name.0)
    }
}

impl<T, B> Shortener for LinkShortener<T, B>
where
    T: NamesRepository,
    B: NameGeneratorTrait,
{
    fn shorten_name(
        &self,
        input: &str,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Result<OutputLink, String> {
        let validated_input = self.validate_input(input)?;
        let mut generated_name = self.generator.make_random_name(rng);
        let mut already_exists = self.names_repo.name_exists(&generated_name)?;
        loop {
            if already_exists == false {
                break;
            }
            generated_name = self.generator.make_random_name(rng);
            already_exists = self.names_repo.name_exists(&generated_name)?;
        }
        self.names_repo
            .store_name(&validated_input, &generated_name)?;
        Ok(self.to_output_link(&generated_name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NameGenerator;
    use rand::Rng;

    struct MockNamesRepository {}
    impl NamesRepository for MockNamesRepository {
        fn store_name(&self, _original: &Url, _generated: &GeneratedName) -> Result<(), String> {
            Ok(())
        }

        fn name_exists(&self, _name: &GeneratedName) -> Result<bool, String> {
            let mut rng = rand::thread_rng();
            let exists = rng.gen_bool(0.5);
            Ok(exists)
        }

        fn retrieve_original_name(&self, _name: &GeneratedName) -> Result<String, String> {
            Ok("".to_string())
        }
    }

    #[test]
    fn test_validate_input() {
        let shortener = LinkShortener::new(
            "http://localhost:8080".to_string(),
            MockNamesRepository {},
            NameGenerator::new(),
        );
        let result = shortener.validate_input("http://localhost:8080");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_input_invalid() {
        let shortener = LinkShortener::new(
            "http://localhost:8080".to_string(),
            MockNamesRepository {},
            NameGenerator::new(),
        );
        let result = shortener.validate_input("google.com");
        assert!(result.is_err());
    }

    #[test]
    fn test_to_output_link() {
        let shortener = LinkShortener::new(
            "http://localhost:8080".to_string(),
            MockNamesRepository {},
            NameGenerator::new(),
        );
        let generated_name = GeneratedName("test".to_string());
        let result = shortener.to_output_link(&generated_name);
        assert_eq!(result.0, "http://localhost:8080/test");
    }

    #[test]
    fn test_generating_link_does_not_hang() {
        // This test is to ensure that the code does not hang when generating a link.
        // This is because there is an infinite loop in the code.
        let shortener = LinkShortener::new(
            "http://localhost:8080".to_string(),
            MockNamesRepository {},
            NameGenerator::new(),
        );
        let mut rng = rand::thread_rng();
        let result = shortener.shorten_name("http://localhost:8080", &mut rng);
        assert!(result.is_ok());
    }
}
