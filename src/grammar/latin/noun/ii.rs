//! Latin Second Declension

use super::*;

/// 2nd declension ending type determined from nominative form
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Ending {
    UM,
    US,
    ER,
    ERI,
    IR,
}

impl NounInstance<'_> {
    fn ending_ii(&self) -> Ending {
        let nom = self.noun.nominative.clone();
        let ending: String = nom.chars().skip(nom.chars().count() - 2).collect();
        let e_stem: Option<char> = self.noun.genitive.chars().rev().nth(2);
        match ending.as_str() {
            "um" => Ending::UM,
            "us" => Ending::US,
            "ir" => Ending::IR,
            "er" => match e_stem {
                Some('e') => Ending::ERI,
                _ => Ending::ER,
            },
            _ => Ending::UM,
        }
    }

    fn stem_ii(&self) -> String {
        let nom = self.noun.nominative.clone();
        let len = nom.chars().count();
        let end = self.ending_ii();
        match end {
            Ending::UM | Ending::US => nom.chars().take(len - 2).collect(),
            Ending::ERI => nom,
            Ending::IR => nom,
            Ending::ER => match (self.case, self.number) {
                (Case::Nominative | Case::Vocative, Number::Singular) => nom,
                _ => self.noun.genitive.chars().take(len - 1).collect(),
            },
        }
    }

    pub(super) fn inflect_ii(&self) -> String {
        let ending = self.ending_ii();
        match self.number {
            Number::Singular => match self.case {
                Case::Nominative => match ending {
                    Ending::UM => "um".to_string(),
                    Ending::US => "us".to_string(),
                    Ending::ER => "".to_string(),
                    Ending::ERI => "".to_string(),
                    Ending::IR => "".to_string(),
                },
                Case::Genitive => "ī".to_string(),
                Case::Dative => "ō".to_string(),
                Case::Accusative => match ending {
                    Ending::UM => "um".to_string(),
                    _ => "um".to_string(),
                },
                Case::Ablative => "ō".to_string(),
                Case::Vocative => match ending {
                    Ending::US => "e".to_string(),
                    Ending::UM => "um".to_string(),
                    Ending::ER => "".to_string(),
                    Ending::ERI => "".to_string(),
                    Ending::IR => "".to_string(),
                },
            },
            Number::Plural => match self.case {
                Case::Nominative => match ending {
                    Ending::UM => "a".to_string(),
                    _ => "ī".to_string(),
                },
                Case::Genitive => "ōrum".to_string(),
                Case::Dative => "īs".to_string(),
                Case::Accusative => match ending {
                    Ending::UM => "a".to_string(),
                    _ => "ōs".to_string(),
                },
                Case::Ablative => "īs".to_string(),
                Case::Vocative => match ending {
                    Ending::UM => "a".to_string(),
                    _ => "ī".to_string(),
                },
            },
        }
    }

    pub(super) fn decline_ii(&self) -> String {
        let stem = self.stem_ii();
        let inflection = self.inflect_ii();
        format!("{}{}", stem, inflection)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_noun_ii() {
        let noun = Noun {
            id: None,
            declension: Declension::II,
            nominative: "circus".to_string(),
            genitive: "circī".to_string(),
            gender: Gender::Masculine,
        };

        assert_eq!(noun.genitive, "circī");
        assert_eq!(noun.gender, Gender::Masculine);
    }

    #[rstest]
    #[case(Case::Nominative, Number::Singular, "pelagus")]
    #[case(Case::Genitive, Number::Singular, "pelagī")]
    #[case(Case::Dative, Number::Singular, "pelagō")]
    #[case(Case::Accusative, Number::Singular, "pelagum")]
    #[case(Case::Ablative, Number::Singular, "pelagō")]
    #[case(Case::Vocative, Number::Singular, "pelage")]
    #[case(Case::Nominative, Number::Plural, "pelagī")]
    #[case(Case::Genitive, Number::Plural, "pelagōrum")]
    #[case(Case::Dative, Number::Plural, "pelagīs")]
    #[case(Case::Accusative, Number::Plural, "pelagōs")]
    #[case(Case::Ablative, Number::Plural, "pelagīs")]
    #[case(Case::Vocative, Number::Plural, "pelagī")]
    fn test_decline_ii_us(#[case] case: Case, #[case] number: Number, #[case] expected: String) {
        let noun = Noun {
            id: None,
            declension: Declension::II,
            nominative: "pelagus".to_string(),
            genitive: "pelagī".to_string(),
            gender: Gender::Masculine,
        };
        let ni = NounInstance {
            noun: &noun,
            case,
            number,
        };
        assert_eq!(expected, ni.decline());
    }

    #[rstest]
    #[case(Case::Nominative, Number::Singular, "bellum")]
    #[case(Case::Genitive, Number::Singular, "bellī")]
    #[case(Case::Dative, Number::Singular, "bellō")]
    #[case(Case::Accusative, Number::Singular, "bellum")]
    #[case(Case::Ablative, Number::Singular, "bellō")]
    #[case(Case::Vocative, Number::Singular, "bellum")]
    #[case(Case::Nominative, Number::Plural, "bella")]
    #[case(Case::Genitive, Number::Plural, "bellōrum")]
    #[case(Case::Dative, Number::Plural, "bellīs")]
    #[case(Case::Accusative, Number::Plural, "bella")]
    #[case(Case::Ablative, Number::Plural, "bellīs")]
    #[case(Case::Vocative, Number::Plural, "bella")]
    fn test_decline_ii_um(#[case] case: Case, #[case] number: Number, #[case] expected: String) {
        let noun = Noun {
            id: None,
            declension: Declension::II,
            nominative: "bellum".to_string(),
            genitive: "bellī".to_string(),
            gender: Gender::Neuter,
        };
        let ni = NounInstance {
            noun: &noun,
            case,
            number,
        };
        assert_eq!(expected, ni.decline());
    }

    #[rstest]
    #[case(Case::Nominative, Number::Singular, "puer")]
    #[case(Case::Genitive, Number::Singular, "puerī")]
    #[case(Case::Dative, Number::Singular, "puerō")]
    #[case(Case::Accusative, Number::Singular, "puerum")]
    #[case(Case::Ablative, Number::Singular, "puerō")]
    #[case(Case::Vocative, Number::Singular, "puer")]
    #[case(Case::Nominative, Number::Plural, "puerī")]
    #[case(Case::Genitive, Number::Plural, "puerōrum")]
    #[case(Case::Dative, Number::Plural, "puerīs")]
    #[case(Case::Accusative, Number::Plural, "puerōs")]
    #[case(Case::Ablative, Number::Plural, "puerīs")]
    #[case(Case::Vocative, Number::Plural, "puerī")]
    fn test_decline_ii_eri(#[case] case: Case, #[case] number: Number, #[case] expected: String) {
        let noun = Noun {
            id: None,
            declension: Declension::II,
            nominative: "puer".to_string(),
            genitive: "puerī".to_string(),
            gender: Gender::Masculine,
        };
        let ni = NounInstance {
            noun: &noun,
            case,
            number,
        };
        assert_eq!(expected, ni.decline());
    }

    #[rstest]
    #[case(Case::Nominative, Number::Singular, "aper")]
    #[case(Case::Genitive, Number::Singular, "aprī")]
    #[case(Case::Dative, Number::Singular, "aprō")]
    #[case(Case::Accusative, Number::Singular, "aprum")]
    #[case(Case::Ablative, Number::Singular, "aprō")]
    #[case(Case::Vocative, Number::Singular, "aper")]
    #[case(Case::Nominative, Number::Plural, "aprī")]
    #[case(Case::Genitive, Number::Plural, "aprōrum")]
    #[case(Case::Dative, Number::Plural, "aprīs")]
    #[case(Case::Accusative, Number::Plural, "aprōs")]
    #[case(Case::Ablative, Number::Plural, "aprīs")]
    #[case(Case::Vocative, Number::Plural, "aprī")]
    fn test_decline_ii_er(#[case] case: Case, #[case] number: Number, #[case] expected: String) {
        let noun = Noun {
            id: None,
            declension: Declension::II,
            nominative: "aper".to_string(),
            genitive: "aprī".to_string(),
            gender: Gender::Masculine,
        };
        let ni = NounInstance {
            noun: &noun,
            case,
            number,
        };
        assert_eq!(expected, ni.decline());
    }

    #[rstest]
    #[case(Case::Nominative, Number::Singular, "deus")]
    #[case(Case::Genitive, Number::Singular, "deī")]
    #[case(Case::Dative, Number::Singular, "deō")]
    #[case(Case::Accusative, Number::Singular, "deum")]
    #[case(Case::Ablative, Number::Singular, "deō")]
    #[case(Case::Vocative, Number::Singular, "dee")] // non attestata
    #[case(Case::Nominative, Number::Plural, "deī")]
    #[case(Case::Genitive, Number::Plural, "deōrum")]
    #[case(Case::Dative, Number::Plural, "deīs")]
    #[case(Case::Accusative, Number::Plural, "deōs")]
    #[case(Case::Ablative, Number::Plural, "deīs")]
    #[case(Case::Vocative, Number::Plural, "deī")]
    /// TODO: alt-forms - there are unusual forms of deus not accounted for here
    fn test_decline_ii_deus(#[case] case: Case, #[case] number: Number, #[case] expected: String) {
        let noun = Noun {
            id: None,
            declension: Declension::II,
            nominative: "deus".to_string(),
            genitive: "deī".to_string(),
            gender: Gender::Masculine,
        };
        let ni = NounInstance {
            noun: &noun,
            case,
            number,
        };
        assert_eq!(expected, ni.decline());
    }

    #[rstest]
    #[case(Case::Nominative, Number::Singular, "vir")]
    #[case(Case::Genitive, Number::Singular, "virī")]
    #[case(Case::Dative, Number::Singular, "virō")]
    #[case(Case::Accusative, Number::Singular, "virum")]
    #[case(Case::Ablative, Number::Singular, "virō")]
    #[case(Case::Vocative, Number::Singular, "vir")]
    #[case(Case::Nominative, Number::Plural, "virī")]
    #[case(Case::Genitive, Number::Plural, "virōrum")]
    #[case(Case::Dative, Number::Plural, "virīs")]
    #[case(Case::Accusative, Number::Plural, "virōs")]
    #[case(Case::Ablative, Number::Plural, "virīs")]
    #[case(Case::Vocative, Number::Plural, "virī")]
    fn test_decline_ii_vir(#[case] case: Case, #[case] number: Number, #[case] expected: String) {
        let noun = Noun {
            id: None,
            declension: Declension::II,
            nominative: "vir".to_string(),
            genitive: "virī".to_string(),
            gender: Gender::Masculine,
        };
        let ni = NounInstance {
            noun: &noun,
            case,
            number,
        };
        assert_eq!(expected, ni.decline());
    }
}
