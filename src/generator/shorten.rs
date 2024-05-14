use core::fmt;
use serde::Serialize;

use crate::errors::AppError;

use super::{
    database::NamesRepository,
    name_generator::{GeneratedName, NameGeneratorTrait},
};
use url::Url;

#[derive(Serialize)]
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
        name: &mut String,
        names_repo: &mut impl NamesRepository,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Result<OutputLink, AppError>;

    fn get_original_name(
        &self,
        shortened_link: &str,
        names_repo: &mut impl NamesRepository,
    ) -> Result<String, AppError>;
}

pub struct ShortenService<'a, 'b, B>
where
    B: NameGeneratorTrait,
{
    base_url: &'a str,
    generator: &'b B,
}

impl<'a, 'b, B> ShortenService<'a, 'b, B>
where
    B: NameGeneratorTrait,
{
    pub fn new(base_url: &'a str, generator: &'b B) -> Self {
        Self {
            base_url,
            generator,
        }
    }

    fn validate_input(&self, input_link: &mut String) -> Result<Url, AppError> {
        let error_msg =  "You supplied an invalid link. Are you sure its a valid URL? TIP: it should either not have an scheme or be HTTPS".to_string();
        let maybe_url = if input_link.starts_with("https://") {
            Url::parse(input_link)
        } else {
            input_link.insert_str(0, "https://");
            Url::parse(input_link)
        };

        maybe_url.map_err(|_| AppError::UserInputError(error_msg))
    }

    fn to_output_link(&self, generated_name: GeneratedName) -> OutputLink {
        let mut link = generated_name.0;
        link.insert_str(0, self.base_url);
        OutputLink { link }
    }
}

impl<'a, 'b, B> Shortener for ShortenService<'a, 'b, B>
where
    B: NameGeneratorTrait,
{
    fn shorten_name(
        &self,
        input: &mut String,
        names_repo: &mut impl NamesRepository,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Result<OutputLink, AppError> {
        let validated_input = self.validate_input(input)?;
        let mut generated_name = self.generator.make_random_name(rng);
        let mut already_exists = names_repo.name_exists(&generated_name)?;
        loop {
            if !already_exists {
                break;
            }
            generated_name = self.generator.make_random_name(rng);
            already_exists = names_repo.name_exists(&generated_name)?;
        }
        names_repo.store_name(&validated_input, &generated_name)?;
        Ok(self.to_output_link(generated_name))
    }

    fn get_original_name(
        &self,
        shortened_link: &str,
        names_repo: &mut impl NamesRepository,
    ) -> Result<String, AppError> {
        let generated_name = GeneratedName(shortened_link.to_string());
        names_repo.retrieve_original_name(&generated_name)
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
        ) -> Result<(), AppError> {
            Ok(())
        }

        fn name_exists(&mut self, _name: &GeneratedName) -> Result<bool, AppError> {
            let mut rng = rand::thread_rng();
            let exists = rng.gen_bool(0.5);
            Ok(exists)
        }

        fn retrieve_original_name(&mut self, _name: &GeneratedName) -> Result<String, AppError> {
            Ok("".to_string())
        }
    }

    #[test]
    fn test_validate_input() {
        let generator = NameGenerator::default();
        let shortener = ShortenService::new("http://localhost:8080/", &generator);
        let result = shortener.validate_input(&mut "https://localhost:8080/".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_http_is_invalid() {
        let generator = NameGenerator::default();
        let shortener = ShortenService::new("http://localhost:8080/", &generator);
        let result = shortener.validate_input(&mut "http://localhost:8080/".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_input_invalid() {
        let generator = NameGenerator::default();

        let shortener = ShortenService::new("http://localhost:8080/", &generator);
        let result = shortener.validate_input(&mut "google.com".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_to_output_link() {
        let generator = NameGenerator::default();
        let shortener = ShortenService::new("http://localhost:8080/", &generator);
        let generated_name = GeneratedName("test".to_string());
        let result = shortener.to_output_link(generated_name);
        assert_eq!(result.link, "http://localhost:8080/test");
    }

    #[test]
    fn test_generating_link_does_not_hang() {
        // This test is to ensure that the code does not hang when generating a link.
        // This is because there is an infinite loop in the code.
        let mut repo = MockNamesRepository {};
        let generator = NameGenerator::default();

        let shortener = ShortenService::new("http://localhost:8080/", &generator);
        let mut rng = rand::thread_rng();
        let result = shortener.shorten_name(
            &mut "https://localhost:8080/".to_string(),
            &mut repo,
            &mut rng,
        );
        assert!(result.is_ok());
    }
}
