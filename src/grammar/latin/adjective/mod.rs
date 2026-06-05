use crate::grammar::latin::{Number, noun::*};

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum AdjDeclension {
    I_II,
    III,
}

#[derive(Debug)]
pub struct Adjective {
    pub declension: AdjDeclension,
    pub f: String,
    pub m: String,
    pub n: String,
}

pub struct AdjectiveInstance<'ad> {
    pub adjective: &'ad Adjective,
    pub case: Case,
    pub number: Number,
    pub gender: Gender,
}

impl<'ad> AdjectiveInstance<'ad> {
    pub fn decline(&self) -> String {
        let declension = match self.adjective.declension {
            AdjDeclension::I_II => match self.gender {
                Gender::Feminine => Declension::I,
                _ => Declension::II,
            },
            AdjDeclension::III => Declension::III,
        };
        let nominative = match self.gender {
            Gender::Feminine => self.adjective.f.clone(),
            Gender::Masculine => self.adjective.m.clone(),
            Gender::Neuter => self.adjective.n.clone(),
        };
        let mut geni: String = nominative
            .clone()
            .chars()
            .take(nominative.chars().count() - if nominative.ends_with("a") { 1 } else { 2 })
            .collect();
        geni.push_str(match self.adjective.declension {
            AdjDeclension::I_II => match self.gender {
                Gender::Feminine => "ae",
                _ => "ī",
            },
            AdjDeclension::III => "is",
        });

        let noun = Noun {
            id: None,
            declension,
            nominative,
            genitive: geni,
            gender: self.gender,
        };
        let ni = NounInstance {
            noun: &noun,
            case: self.case,
            number: self.number,
        };

        let mut nid = ni.decline();
        // Handle adjectival abl.s. w/o changing NounInstance.decline
        match (noun.declension, self.case, self.number) {
            (Declension::III, Case::Ablative, Number::Singular) => {
                nid.pop();
                nid.push('ī');
                nid
            }
            _ => nid,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_noun_i() {
        let adj = Adjective {
            declension: AdjDeclension::I_II,
            f: "nova".to_string(),
            m: "novus".to_string(),
            n: "novum".to_string(),
        };

        assert_eq!(adj.f, "nova");
        assert_eq!(adj.declension, AdjDeclension::I_II);
    }

    #[rstest]
    #[case(Case::Nominative, Number::Singular, "nova")]
    #[case(Case::Genitive, Number::Singular, "novae")]
    #[case(Case::Dative, Number::Singular, "novae")]
    #[case(Case::Accusative, Number::Singular, "novam")]
    #[case(Case::Ablative, Number::Singular, "novā")]
    #[case(Case::Vocative, Number::Singular, "nova")]
    #[case(Case::Nominative, Number::Plural, "novae")]
    #[case(Case::Genitive, Number::Plural, "novārum")]
    #[case(Case::Dative, Number::Plural, "novīs")]
    #[case(Case::Accusative, Number::Plural, "novās")]
    #[case(Case::Ablative, Number::Plural, "novīs")]
    #[case(Case::Vocative, Number::Plural, "novae")]
    fn test_decline_adj_i_f(#[case] case: Case, #[case] number: Number, #[case] expected: String) {
        let adj = Adjective {
            declension: AdjDeclension::I_II,
            f: "nova".to_string(),
            m: "novus".to_string(),
            n: "novum".to_string(),
        };
        let ai = AdjectiveInstance {
            adjective: &adj,
            case,
            number,
            gender: Gender::Feminine,
        };
        assert_eq!(expected, ai.decline());
    }

    #[rstest]
    #[case(Case::Nominative, Number::Singular, "novus")]
    #[case(Case::Genitive, Number::Singular, "novī")]
    #[case(Case::Dative, Number::Singular, "novō")]
    #[case(Case::Accusative, Number::Singular, "novum")]
    #[case(Case::Ablative, Number::Singular, "novō")]
    #[case(Case::Vocative, Number::Singular, "nove")]
    #[case(Case::Nominative, Number::Plural, "novī")]
    #[case(Case::Genitive, Number::Plural, "novōrum")]
    #[case(Case::Dative, Number::Plural, "novīs")]
    #[case(Case::Accusative, Number::Plural, "novōs")]
    #[case(Case::Ablative, Number::Plural, "novīs")]
    #[case(Case::Vocative, Number::Plural, "novī")]
    fn test_decline_adj_ii_m(#[case] case: Case, #[case] number: Number, #[case] expected: String) {
        let adj = Adjective {
            declension: AdjDeclension::I_II,
            f: "nova".to_string(),
            m: "novus".to_string(),
            n: "novum".to_string(),
        };
        let ai = AdjectiveInstance {
            adjective: &adj,
            case,
            number,
            gender: Gender::Masculine,
        };
        assert_eq!(expected, ai.decline());
    }

    #[rstest]
    #[case(Case::Nominative, Number::Singular, "novum")]
    #[case(Case::Genitive, Number::Singular, "novī")]
    #[case(Case::Dative, Number::Singular, "novō")]
    #[case(Case::Accusative, Number::Singular, "novum")]
    #[case(Case::Ablative, Number::Singular, "novō")]
    #[case(Case::Vocative, Number::Singular, "novum")]
    #[case(Case::Nominative, Number::Plural, "nova")]
    #[case(Case::Genitive, Number::Plural, "novōrum")]
    #[case(Case::Dative, Number::Plural, "novīs")]
    #[case(Case::Accusative, Number::Plural, "nova")]
    #[case(Case::Ablative, Number::Plural, "novīs")]
    #[case(Case::Vocative, Number::Plural, "nova")]
    fn test_decline_adj_ii_n(#[case] case: Case, #[case] number: Number, #[case] expected: String) {
        let adj = Adjective {
            declension: AdjDeclension::I_II,
            f: "nova".to_string(),
            m: "novus".to_string(),
            n: "novum".to_string(),
        };
        let ai = AdjectiveInstance {
            adjective: &adj,
            case,
            number,
            gender: Gender::Neuter,
        };
        assert_eq!(expected, ai.decline());
    }

    #[rstest]
    #[case(Case::Nominative, Number::Singular, "omnis")]
    #[case(Case::Genitive, Number::Singular, "omnis")]
    #[case(Case::Dative, Number::Singular, "omnī")]
    #[case(Case::Accusative, Number::Singular, "omnem")]
    #[case(Case::Ablative, Number::Singular, "omnī")]
    #[case(Case::Vocative, Number::Singular, "omnis")]
    #[case(Case::Nominative, Number::Plural, "omnēs")]
    #[case(Case::Genitive, Number::Plural, "omnium")]
    #[case(Case::Dative, Number::Plural, "omnibus")]
    #[case(Case::Accusative, Number::Plural, "omnēs")]
    #[case(Case::Ablative, Number::Plural, "omnibus")]
    #[case(Case::Vocative, Number::Plural, "omnēs")]
    fn test_decline_adj_iii_f(
        #[case] case: Case,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let adj = Adjective {
            declension: AdjDeclension::III,
            f: "omnis".to_string(),
            m: "omnis".to_string(),
            n: "omne".to_string(),
        };
        let ai = AdjectiveInstance {
            adjective: &adj,
            case,
            number,
            gender: Gender::Feminine,
        };
        assert_eq!(expected, ai.decline());
    }
}
