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
        match url {
            Ok(valid_link) => Ok(valid_link),
            Err(_) => Err("Invalid URL".to_string()),
        }
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
