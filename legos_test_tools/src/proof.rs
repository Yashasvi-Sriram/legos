#[derive(Debug)]
pub enum CorrectnessProof {
    TODO,
    Inferred,
    Because(String),
}

#[derive(Debug)]
pub enum BigO {
    C,
    N,
    NLogN,
    NN,
    NNN,
    TwoToN,
}

#[derive(Debug)]
pub enum ComplexityProof {
    TODO,
    Inferred(BigO),
    Because(String, BigO),
}
