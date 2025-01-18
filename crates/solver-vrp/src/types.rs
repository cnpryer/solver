pub type Id = String;
pub type Index = usize;

pub enum Number {
    Integer(Integer),
    Float(Float),
}

#[derive(Clone, Copy)]
pub enum Integer {
    I64(i64),
}

#[derive(Clone, Copy)]
pub enum Float {
    F64(f64),
}
