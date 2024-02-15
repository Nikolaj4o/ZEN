use std::time::Instant;
use pedersen_example::*;
use ark_serialize::CanonicalDeserialize;
use ark_serialize::CanonicalSerialize;
use ark_poly::univariate::DensePolynomial;
use ark_poly_commit::kzg10::Powers;
use ark_poly::Polynomial;
use ark_poly::UVPolynomial;
use ark_std::borrow::Cow;
use ark_std::One;
use std::ops::Div;
use ark_ff::UniformRand;
use ark_ff::*;
use ark_groth16::*;
use ark_crypto_primitives::{commitment::pedersen::Randomness, SNARK};
use ark_bls12_381::Bls12_381;
use crate::full_circuit::convert_2d_vector_into_1d;
use ark_std::test_rng;
use ark_sponge::{ CryptographicSponge, FieldBasedCryptographicSponge, poseidon::PoseidonSponge};
use ark_ed_on_bls12_381::{constraints::FqVar, Fq};

fn kzg_proof(
    poly: DensePolynomial<Fq>,
    //pp: ark_poly_commit::kzg10::UniversalParams<E>,
    powers: Powers<Bls12_381>,
    x: Fq,
) -> (Fq, ark_poly_commit::kzg10::Commitment<Bls12_381>, ark_poly_commit::kzg10::Commitment<Bls12_381>)  {
    let rng = &mut ark_std::test_rng();

    let (commit, rand) =
    ark_poly_commit::kzg10::KZG10::commit(&powers, &poly, None, None).unwrap();
    let eval = poly.evaluate(&x);
    let eval_poly = DensePolynomial::<Fq>::from_coefficients_vec(vec![-eval]);
    let divisor_poly = DensePolynomial::<Fq>::from_coefficients_vec(vec![-x, Fq::one()]);
    let wintess_poly = (poly + eval_poly).div(&divisor_poly);
    let (witness_commit, rand) = 
    ark_poly_commit::kzg10::KZG10::commit(&powers, &wintess_poly, None, None).unwrap();

    (eval, commit, witness_commit)
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let mut rng = test_rng();
    //let (x, l1_mat, l2_mat): (Vec<u8>, Vec<Vec<u8>>, Vec<Vec<u8>>) = read_shallownet_inputs_u8();
    let x: Vec<u8> = read_vector1d("pretrained_model/shallownet/X_q.txt".to_string(), 784); // only read one image
    let l1_mat: Vec<Vec<u8>> = read_vector2d(
        "pretrained_model/shallownet/l1_weight_q.txt".to_string(),
        128,
        784,
    );
    let l2_mat: Vec<Vec<u8>> = read_vector2d(
        "pretrained_model/shallownet/l2_weight_q.txt".to_string(),
        10,
        128,
    );
    let x_0: Vec<u8> = read_vector1d("pretrained_model/shallownet/X_z.txt".to_string(), 1);
    let l1_output_0: Vec<u8> =
        read_vector1d("pretrained_model/shallownet/l1_output_z.txt".to_string(), 1);
    let l2_output_0: Vec<u8> =
        read_vector1d("pretrained_model/shallownet/l2_output_z.txt".to_string(), 1);
    let l1_mat_0: Vec<u8> =
        read_vector1d("pretrained_model/shallownet/l1_weight_z.txt".to_string(), 1);
    let l2_mat_0: Vec<u8> =
        read_vector1d("pretrained_model/shallownet/l2_weight_z.txt".to_string(), 1);

    let l1_mat_multiplier: Vec<f32> = read_vector1d_f32(
        "pretrained_model/shallownet/l1_weight_s.txt".to_string(),
        128,
    );
    let l2_mat_multiplier: Vec<f32> = read_vector1d_f32(
        "pretrained_model/shallownet/l2_weight_s.txt".to_string(),
        10,
    );

    //println!("zero points x_0 {}  l1_out_0 {} l2_out_0 {}, l1_mat_0 {}, l2_mat_0 {}", x_0[0], l1_output_0[0], l2_output_0[0], l1_mat_0[0], l2_mat_0[0]);

    let z: Vec<u8> = full_circuit_forward_u8(
        x.clone(),
        l1_mat.clone(),
        l2_mat.clone(),
        x_0[0],
        l1_output_0[0],
        l2_output_0[0],
        l1_mat_0[0],
        l2_mat_0[0],
        l1_mat_multiplier.clone(),
        l2_mat_multiplier.clone(),
    );
    let begin = Instant::now();
    let mut commit_vec = Vec::<Fq>::new();
    let x_fq: Vec<Fq> = x.clone().iter().map(|x| (*x).into()).collect();
    commit_vec.extend(x_fq);
    let l1_fq: Vec<Fq> = convert_2d_vector_into_1d(l1_mat.clone()).iter().map(|x| (*x).into()).collect();
    commit_vec.extend(l1_fq);
    let l2_fq: Vec<Fq> = convert_2d_vector_into_1d(l2_mat.clone()).iter().map(|x| (*x).into()).collect();
    commit_vec.extend(l2_fq);
    let z_fq: Vec<Fq> = z.clone().iter().map(|x| (*x).into()).collect();
    commit_vec.extend(z_fq);
    let commit_len: usize = commit_vec.len();

    let mut rng = &mut ark_std::test_rng();
    let pp = ark_poly_commit::kzg10::KZG10::<
        ark_bls12_381::Bls12_381,
        DensePolynomial<Fq>,
    >::setup(commit_len - 1, true, rng)
    .unwrap();

    let powers_of_gamma_g = (0..commit_len)
        .map(|i| pp.powers_of_gamma_g[&i])
        .collect::<Vec<_>>();

    let powers = ark_poly_commit::kzg10::Powers::<ark_bls12_381::Bls12_381> {
        powers_of_g: Cow::Borrowed(&pp.powers_of_g),
        powers_of_gamma_g: Cow::Owned(powers_of_gamma_g),
    };

    let beta = Fq::rand(rng);
    let mut powers_of_beta: Vec<Fq> = vec![1u8.into()];
    let mut cur = beta;

    for _ in 1..commit_len {
        powers_of_beta.push(cur);
        cur *= &beta;
    }
    let commit_poly = DensePolynomial::<Fq>::from_coefficients_slice(&commit_vec);
    let (rho, c, pi) = kzg_proof(commit_poly, powers.clone(), beta);
    let end = Instant::now();
    println!("commit time {:?}", end.duration_since(begin));
    // println!("x_squeeze len {}", x_squeeze.len());
    // println!("l1_squeeze len {}", l1_squeeze.len());
    // println!("l2_squeeze len {}", l2_squeeze.len());
    // println!("z_squeeze len {}", z_squeeze.len());
    let classification_res = argmax_u8(z.clone());

    let full_circuit = FullCircuitOpLv3PolyClassification {
        x: x.clone(),
        l1: l1_mat,
        l2: l2_mat,
        z: z.clone(),
        argmax_res: classification_res,
        rho,
        powers_of_beta: powers_of_beta.clone(),

        x_0: x_0[0],
        y_0: l1_output_0[0],
        z_0: l2_output_0[0],
        l1_mat_0: l1_mat_0[0],
        l2_mat_0: l2_mat_0[0],
        multiplier_l1: l1_mat_multiplier.clone(),
        multiplier_l2: l2_mat_multiplier.clone(),
    };

    println!("start generating random parameters");
    let begin = Instant::now();

    // pre-computed parameters
    let param =
        generate_random_parameters::<Bls12_381, _, _>(full_circuit.clone(), &mut rng)
            .unwrap();
    let end = Instant::now();
    println!("setup time {:?}", end.duration_since(begin));



    let pvk = prepare_verifying_key(&param.vk);
    println!("random parameters generated!\n");

    // prover
    let begin = Instant::now();
    let proof = create_random_proof(full_circuit, &param, &mut rng).unwrap();
    let end = Instant::now();
    println!("prove time {:?}", end.duration_since(begin));

    let mut inputs = powers_of_beta.clone();
    inputs.push(rho);

    let begin = Instant::now();
    assert!(verify_proof(&pvk, &proof, &inputs[..].as_ref()).unwrap());
    let end = Instant::now();
    println!("verification time {:?}", end.duration_since(begin));
}
