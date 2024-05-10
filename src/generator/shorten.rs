use super::name_generator::{GeneratedName, NameGeneratorTrait};
use url::Url;

pub trait NamesRepository {
    fn store_name(&self, original: &Url, generated: &GeneratedName) -> Result<(), String>;
    fn name_exists(&self, name: &GeneratedName) -> Result<bool, String>;
    fn retrieve_original_name(&self, name: &GeneratedName) -> Result<String, String>;
}

pub trait Shortener {
    fn shorten_name(
        &self,
        name: &Url,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Result<GeneratedName, String>;
}

pub struct LinkShortener<T, B>
where
    T: NamesRepository,
    B: NameGeneratorTrait,
{
    names_repo: T,
    generator: B,
}
impl<T, B> LinkShortener<T, B>
where
    T: NamesRepository,
    B: NameGeneratorTrait,
{
    pub fn new(names_repo: T, generator: B) -> Self {
        Self {
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
}

impl<T, B> Shortener for LinkShortener<T, B>
where
    T: NamesRepository,
    B: NameGeneratorTrait,
{
    fn shorten_name(
        &self,
        input: &Url,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Result<GeneratedName, String> {
        let mut generated_name = self.generator.make_random_name(rng);
        let mut already_exists = self.names_repo.name_exists(&generated_name)?;
        loop {
            if already_exists == false {
                break;
            }
            generated_name = self.generator.make_random_name(rng);
            already_exists = self.names_repo.name_exists(&generated_name)?;
        }
        self.names_repo.store_name(input, &generated_name)?;
        Ok(generated_name)
    }
}
