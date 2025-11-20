
pub trait UsedIn: Sized{
    
    #[inline(always)]
    fn pipe<F: FnOnce(Self) -> O , O>(self, fun: F) -> O{
        fun(self) 
    }
}
impl<T> UsedIn for T where T: Sized{}
pub trait With: Sized{
    #[inline(always)]
    fn with<'a, F: FnOnce(&mut Self)>(mut self, fun: F) -> Self{
        fun(&mut self);
        self
    }
}

impl<T> With for T where T: Sized{}