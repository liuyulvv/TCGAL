use super::base_number_type_trait::BaseNumberTypeTrait;

pub struct Eps<T: BaseNumberTypeTrait> {
    pub value: T,
}

impl<T: BaseNumberTypeTrait> Eps<T> {
    pub fn new(value: T) -> Eps<T> {
        Eps { value }
    }
}

impl<T: BaseNumberTypeTrait> Default for Eps<T> {
    fn default() -> Self {
        Eps {
            value: T::default_eps(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eps() {
        let eps_f32: Eps<f32> = Eps::new(1.0);
        let eps_64: Eps<f64> = Eps::new(1.0);
        assert_eq!(eps_f32.value, 1.0);
        assert_eq!(eps_64.value, 1.0);

        let eps_f32_default: Eps<f32> = Eps::default();
        let eps_64_default: Eps<f64> = Eps::default();
        assert_eq!(eps_f32_default.value, 0.0);
        assert_eq!(eps_64_default.value, 0.0);
    }
}
