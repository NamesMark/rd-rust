use std::error::Error;

extern crate slug;
use convert_case::{Case, Casing};

const MAX_SHORT_SLUG: usize = 16;
const MIN_SHORT_SLUG: usize = 5;

pub fn lowercase(s: String) -> Result<String, Box<dyn Error>> {
    Ok(s.to_lowercase())
}

pub fn uppercase(s: String) -> Result<String, Box<dyn Error>> {
    Ok(s.to_uppercase())
}

pub fn no_spaces(s: String) -> Result<String, Box<dyn Error>> {
    Ok(s.replace(" ", ""))
}

pub fn slugify(s: &str) -> Result<String, Box<dyn Error>> {
    Ok(slug::slugify(s))
}

pub fn short_slugify(string: String) -> Result<String, Box<dyn Error>> {
    let short_slug = slug::slugify(&string).chars().take(MAX_SHORT_SLUG).collect::<String>();
    let mut trimmed_short_slug = short_slug.clone();
    while !trimmed_short_slug.ends_with('-') && !trimmed_short_slug.is_empty() {
        trimmed_short_slug.pop();
    }
    let trimmed_short_slug = trimmed_short_slug.trim_end_matches('-').to_string();
    if trimmed_short_slug.len() < MIN_SHORT_SLUG {
        return Ok(short_slug);
    }
    
    Ok(trimmed_short_slug)
}

pub fn alternating(s: String) -> Result<String, Box<dyn Error>> {
    Ok(s.to_case(Case::Alternating))
}

pub fn leetify(s: String) -> Result<String, Box<dyn Error>> {
    Ok(s.to_uppercase().chars().map(|c| match c {
        'A' => '4',
        'B' => '8',
        'E' => '3',
        'G' => '9',
        'I' => '1',
        'O' => '0',
        'S' => '5',
        'T' => '7',
        _ => c,
    }).collect())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lowercase_test() {
        assert_eq!(lowercase("Crabs can be found in all oceans and in fresh water.".to_string()).unwrap(),
                   "crabs can be found in all oceans and in fresh water.");
    }
    
    #[test]
    fn uppercase_test() {
        assert_eq!(uppercase("There are at least 7,000 species, or kinds, of crab!".to_string()).unwrap(),
                   "THERE ARE AT LEAST 7,000 SPECIES, OR KINDS, OF CRAB!");
    }

    #[test]
    fn no_spaces_test() {
        assert_eq!(no_spaces("Crabs and other crustaceans have a hard covering known as the exoskeleton.".to_string()).unwrap(),
                   "Crabsandothercrustaceanshaveahardcoveringknownastheexoskeleton.");
    }

    #[test]
    fn slugify_test() {
        assert_eq!(slugify("Crabs breathe by using gills, but the gills of land crabs have developed in such a way that they act like lungs.").unwrap(),
                   "crabs-breathe-by-using-gills-but-the-gills-of-land-crabs-have-developed-in-such-a-way-that-they-act-like-lungs");
    }

    #[test]
    fn short_slugify_test() {
        assert_eq!(short_slugify("Crabs can be found in all oceans and in fresh water.".to_string()).unwrap(), "crabs-can-be");
        assert_eq!(short_slugify("Crabsandothercrustaceanshaveahard covering known as the exoskeleton.".to_string()).unwrap(), "crabsandothercru");
        assert_eq!(short_slugify("Although a few baby crabs leave the egg looking like small adults, most do not.".to_string()).unwrap(), "although-a-few");
    }

    #[test]
    fn alternating_test() {
        assert_eq!(alternating("Some types, including the blue crab, the Dungeness crab, and the king crab, are often eaten by humans. Crabs may be sold fresh to restaurants or their meat may be canned.".to_string()).unwrap(),
                   "sOmE tYpEs, InClUdInG tHe BlUe CrAb, ThE dUnGeNeSs CrAb, AnD tHe KiNg CrAb, ArE oFtEn EaTeN bY hUmAnS. cRaBs MaY bE sOlD fReSh To ReStAuRaNtS oR tHeIr MeAt MaY bE cAnNeD.");
    }

    #[test]
    fn leetify_test() {
        assert_eq!(leetify("As the crab grows larger, it seeks a larger shell.".to_string()).unwrap(),
                   "45 7H3 CR48 9R0W5 L4R93R, 17 533K5 4 L4R93R 5H3LL.");
    }
}