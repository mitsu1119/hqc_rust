pub mod galois_field_2m;
pub mod galois_field_2m_elem;
pub mod kat_parser;

pub trait ParentSet {
    type ElementType<'a>
    where
        Self: 'a;
}

pub trait Element {
    type ParentType;
}
