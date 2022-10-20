pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let score = match allergen {
            Allergen::Eggs => 1,
            Allergen::Peanuts => 2,
            Allergen::Shellfish => 4,
            Allergen::Strawberries => 8,
            Allergen::Tomatoes => 16,
            Allergen::Chocolate => 32,
            Allergen::Pollen => 64,
            Allergen::Cats => 128,
        };

        score <= self.score
    }

    pub fn allergies(&mut self) -> Vec<Allergen> {
        if self.score > 255 {
            self.score -= 256;
        }

        let all_allergens = [
            Allergen::Cats,
            Allergen::Pollen,
            Allergen::Chocolate,
            Allergen::Tomatoes,
            Allergen::Strawberries,
            Allergen::Shellfish,
            Allergen::Peanuts,
            Allergen::Eggs,
        ];
        let mut user_allergies: Vec<Allergen> = vec![];
        for item in all_allergens {
            if self.is_allergic_to(&item) {
                let score = match item {
                    Allergen::Eggs => 1,
                    Allergen::Peanuts => 2,
                    Allergen::Shellfish => 4,
                    Allergen::Strawberries => 8,
                    Allergen::Tomatoes => 16,
                    Allergen::Chocolate => 32,
                    Allergen::Pollen => 64,
                    Allergen::Cats => 128,
                };
                self.score -= score;
                user_allergies.push(item);
            };
        }
        user_allergies
    }
}
