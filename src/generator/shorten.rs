use super::{
    database::NamesRepository,
    name_generator::{GeneratedName, NameGeneratorTrait},
};
use url::Url;

pub trait Shortener {
    fn shorten_name(
        &self,
        name: &str,
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
        input: &str,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Result<GeneratedName, String> {
        let valid_url = self.validate_input(input)?;
        let mut generated_name = self.generator.make_random_name(rng);
        let mut already_exists = self.names_repo.name_exists(&generated_name)?;
        loop {
            if already_exists == false {
                break;
            }
            generated_name = self.generator.make_random_name(rng);
            already_exists = self.names_repo.name_exists(&generated_name)?;
        }
        self.names_repo.store_name(&valid_url, &generated_name)?;
        Ok(generated_name)
    }
}
