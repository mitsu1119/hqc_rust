use crate::hqc::{hqc_param::HQCParam, hqc_pke::HQC_PKE};

#[allow(non_camel_case_types)]
pub struct HQC_KEM<'a> {
    pke: HQC_PKE<'a>,
}

impl<'a> HQC_KEM<'a> {
    pub fn new(param: HQCParam<'a>) -> Self {
        Self {
            pke: HQC_PKE::new(param),
        }
    }

    pub fn hqc1() -> Self {
        Self::new(HQCParam::hqc1())
    }

    pub fn hqc3() -> Self {
        Self::new(HQCParam::hqc3())
    }

    pub fn hqc5() -> Self {
        Self::new(HQCParam::hqc5())
    }
}
