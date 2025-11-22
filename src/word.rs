pub type Word = i32;
// pub type Activation = bool;

#[derive(Clone, PartialEq, Eq, Debug, Hash, Copy, Default)]
pub enum Activation{
    #[default]
    Inactive,
    Active,
}

impl Into<Activation> for bool{
    fn into(self) -> Activation {
        match self{
            true  => Activation::Active,
            false => Activation::Inactive,
        }
    }
}

impl Into<bool> for Activation{
    fn into(self) -> bool {
        match self{
            Activation::Active   => true,
            Activation::Inactive => false,
        }
    }
}

pub trait ToWord {
    fn to_word(&self) -> i32;
}
impl ToWord for bool{
    fn to_word(&self) -> i32{
        match self{
            &true  => {!0}
            &false => {0}
        }
    }
}
pub trait ToActivation{
    fn to_activation(&self) -> Activation;
}

impl ToActivation for Word{
    fn to_activation(&self) -> Activation{
        (self != &0).into()
    }
}