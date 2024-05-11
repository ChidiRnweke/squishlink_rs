use core::fmt;

use super::{
    database::NamesRepository,
    name_generator::{GeneratedName, NameGeneratorTrait},
};
use url::Url;

pub struct OutputLink {
    link: String,
}

impl fmt::Display for OutputLink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.link)
    }
}

pub trait Shortener {
    fn shorten_name(
        &self,
        name: &str,
        names_repo: &mut impl NamesRepository,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Result<OutputLink, String>;
}

pub struct ShortenService<B>
where
    B: NameGeneratorTrait,
{
    base_url: String,
    generator: B,
}
impl<B> ShortenService<B>
where
    B: NameGeneratorTrait,
{
    pub fn new(base_url: String, generator: B) -> Self {
        Self {
            base_url,
            generator,
        }
    }

    fn validate_input(&self, input_link: &str) -> Result<Url, String> {
        let url = Url::parse(input_link);
        url.map_err(|_| "Invalid URL".to_string())
    }

    fn to_output_link(&self, generated_name: &GeneratedName) -> OutputLink {
        let link = self.base_url.clone() + "/" + &generated_name.0;
        OutputLink { link }
    }
}

impl<B> Shortener for ShortenService<B>
where
    B: NameGeneratorTrait,
{
    fn shorten_name(
        &self,
        input: &str,
        names_repo: &mut impl NamesRepository,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Result<OutputLink, String> {
        let validated_input = self.validate_input(input)?;
        let mut generated_name = self.generator.make_random_name(rng);
        let mut already_exists = names_repo.name_exists(&generated_name)?;
        loop {
            if already_exists == false {
                break;
            }
            generated_name = self.generator.make_random_name(rng);
            already_exists = names_repo.name_exists(&generated_name)?;
        }
        names_repo.store_name(&validated_input, &generated_name)?;
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
        fn store_name(
            &mut self,
            _original: &Url,
            _generated: &GeneratedName,
        ) -> Result<(), String> {
            Ok(())
        }

        fn name_exists(&mut self, _name: &GeneratedName) -> Result<bool, String> {
            let mut rng = rand::thread_rng();
            let exists = rng.gen_bool(0.5);
            Ok(exists)
        }

        fn retrieve_original_name(&mut self, _name: &GeneratedName) -> Result<String, String> {
            Ok("".to_string())
        }
    }

    #[test]
    fn test_validate_input() {
        let shortener =
            ShortenService::new("http://localhost:8080".to_string(), NameGenerator::new());
        let result = shortener.validate_input("http://localhost:8080");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_input_invalid() {
        let shortener =
            ShortenService::new("http://localhost:8080".to_string(), NameGenerator::new());
        let result = shortener.validate_input("google.com");
        assert!(result.is_err());
    }

    #[test]
    fn test_to_output_link() {
        let shortener =
            ShortenService::new("http://localhost:8080".to_string(), NameGenerator::new());
        let generated_name = GeneratedName("test".to_string());
        let result = shortener.to_output_link(&generated_name);
        assert_eq!(result.link, "http://localhost:8080/test");
    }

    #[test]
    fn test_generating_link_does_not_hang() {
        // This test is to ensure that the code does not hang when generating a link.
        // This is because there is an infinite loop in the code.
        let mut repo = MockNamesRepository {};
        let shortener =
            ShortenService::new("http://localhost:8080".to_string(), NameGenerator::new());
        let mut rng = rand::thread_rng();
        let result = shortener.shorten_name("http://localhost:8080", &mut repo, &mut rng);
        assert!(result.is_ok());
    }
}
