use crate::*;
use ark_r1cs_std::alloc::AllocVar;
use ark_r1cs_std::fields::fp::FpVar;
use ark_r1cs_std::boolean::*;
use ark_r1cs_std::prelude::*;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_std::{println, vec, vec::Vec};
use ark_ed_on_bls12_381::{constraints::FqVar, Fq};
use ark_ff::*;
use core::cmp::Ordering;

//used in ORL face recognition problem
#[derive(Debug, Clone)]
pub struct CosineSimilarityCircuitU8 {
    pub vec1: Vec<u8>,
    pub vec2: Vec<u8>,
    pub threshold: u8,
    pub result: bool,
}

impl ConstraintSynthesizer<Fq> for CosineSimilarityCircuitU8 {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fq>) -> Result<(), SynthesisError> {
        let _cir_number = cs.num_constraints();

        let res = Boolean::<Fq>::constant(self.result);

        let norm1 = scala_cs_helper_u8(cs.clone(), &self.vec1.clone(), &self.vec1.clone());
        let norm2 = scala_cs_helper_u8(cs.clone(), &self.vec2.clone(), &self.vec2.clone());
        let numerator = scala_cs_helper_u8(cs.clone(), &self.vec1.clone(), &self.vec2.clone());
        let ten_thousand: Fq = (10000u64).into();
        let ten_thousand_const = FpVar::<Fq>::Constant(ten_thousand);

        let threshold_fq: Fq = self.threshold.into();
        let threshold_const = FpVar::<Fq>::Constant(threshold_fq);
        let left = ten_thousand_const * numerator.clone() * numerator.clone();
        let right = threshold_const.clone() * threshold_const.clone() * norm2 * norm1;

        if res.value().unwrap_or_default() == true {
            left.enforce_cmp(&right, Ordering::Greater, false).unwrap();
        } else {
            left.enforce_cmp(&right, Ordering::Less, true).unwrap();
        }
        println!(
            "CosineSimilarity {} {}",
            cs.num_constraints() - _cir_number,
            cs.num_constraints()
        );
        Ok(())
    }
}

fn mul_cs_helper_u8(cs: ConstraintSystemRef<Fq>, a: u8, c: u8) -> FqVar {
    let aa: Fq = a.into();
    let cc: Fq = c.into();
    let a_var = FpVar::<Fq>::new_witness(ark_relations::ns!(cs, "a gadget"), || Ok(aa)).unwrap();
    let c_var = FpVar::<Fq>::new_witness(ark_relations::ns!(cs, "c gadget"), || Ok(cc)).unwrap();
    a_var * c_var
}

fn scala_cs_helper_u8(cs: ConstraintSystemRef<Fq>, vec1: &[u8], vec2: &[u8]) -> FqVar {
    let _no_cs = cs.num_constraints();
    if vec1.len() != vec2.len() {
        panic!("scala mul: length not equal");
    }
    let mut res =
        FpVar::<Fq>::new_witness(ark_relations::ns!(cs, "q1*q2 gadget"), || Ok(Fq::zero())).unwrap();

    for i in 0..vec1.len() {
        res += mul_cs_helper_u8(cs.clone(), vec1[i], vec2[i]);
    }

    res
}