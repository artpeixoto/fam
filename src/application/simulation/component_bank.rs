pub struct ComponentBank<
    InnerComp, 
    const COMPONENT_COUNT: usize, 
> 
where InnerComp: Sized {
    pub components: Box<[InnerComp; COMPONENT_COUNT]>
}
