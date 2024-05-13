use core::fmt;
use rand::{seq::SliceRandom, Rng};
use std::fs;

#[derive(Debug)]
pub struct GeneratedName(pub String);

impl fmt::Display for GeneratedName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub trait NameGeneratorTrait {
    fn make_random_name(&self, rng: &mut rand::rngs::ThreadRng) -> GeneratedName;
}

pub struct NameGenerator {
    adjectives: Vec<String>,
    nouns: Vec<String>,
}

impl Default for NameGenerator {
    fn default() -> Self {
        let adjectives = read_data("data/animals.txt");
        let nouns = read_data("data/adjectives.txt");
        Self { adjectives, nouns }
    }
}

fn read_data(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Could not read {path}"))
        .lines()
        .map(|s| s.to_string())
        .collect()
}

impl NameGeneratorTrait for NameGenerator {
    fn make_random_name(&self, rng: &mut rand::rngs::ThreadRng) -> GeneratedName {
        // We can safely expect here because we know that the vectors are not empty
        let random_noun = self
            .nouns
            .choose(rng)
            .expect("There are no nouns to generate from");

        let random_adjective = self
            .adjectives
            .choose(rng)
            .expect("There are no adjectives to generate from");

        let random_number = rng.gen_range(0..1000);
        let title_adjective = make_title_case(random_adjective);
        GeneratedName(title_adjective + random_noun + &random_number.to_string())
    }
}

fn make_title_case(random_adjective: &String) -> String {
    let mut adjective = String::from(random_adjective);
    adjective = adjective.remove(0).to_uppercase().to_string() + &adjective;
    adjective
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_title_case() {
        let adjective = String::from("big");
        let result = make_title_case(&adjective);
        assert_eq!(result, "Big");
    }

    #[test]
    fn test_construct_name_generator() {
        let generator = NameGenerator::default();
        assert!(!generator.adjectives.is_empty());
        assert!(!generator.nouns.is_empty());
    }

    #[test]
    fn test_generate_name() {
        let mut rng = rand::thread_rng();
        let generator = NameGenerator::default();
        let name = generator.make_random_name(&mut rng);
        assert!(!name.0.is_empty());
    }
}
