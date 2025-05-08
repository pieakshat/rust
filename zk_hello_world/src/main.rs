use ark_bn254::{Bn254, Fr}; // curve + scalar field
use ark_ff::PrimeField;
use ark_groth16::{
    create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof, Groth16,
    ProvingKey, VerifyingKey,
};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError, Variable};
use rand::thread_rng;

////////////////////////////////////////
// 1.  The constraint system
////////////////////////////////////////
struct AddCircuit<F: PrimeField> {
    pub a: Option<F>, // private
    pub b: Option<F>, // private
    pub c: Option<F>, // *public* (will be an input)
}

impl<F: PrimeField> ConstraintSynthesizer<F> for AddCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        // Allocate private witnesses
        let a_var = cs.new_witness_variable(|| self.a.ok_or(SynthesisError::AssignmentMissing))?;
        let b_var = cs.new_witness_variable(|| self.b.ok_or(SynthesisError::AssignmentMissing))?;

        // Allocate public input
        let c_var = cs.new_input_variable(|| self.c.ok_or(SynthesisError::AssignmentMissing))?;

        // Enforce a + b = c
        // R1CS form: (a + b) * 1 = c
        cs.enforce_constraint(
            ark_relations::r1cs::lc!() + a_var + b_var, // left
            ark_relations::r1cs::lc!() + Variable::One, // right
            ark_relations::r1cs::lc!() + c_var,         // output
        )?;
        Ok(())
    }
}

////////////////////////////////////////
// 2.  Setup, prove, verify
////////////////////////////////////////
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // We’ll prove that 4 + 2 = 6
    let circuit = AddCircuit::<Fr> {
        a: Some(Fr::from(4u64)),
        b: Some(Fr::from(2u64)),
        c: Some(Fr::from(6u64)),
    };

    // ---------- Trusted setup (one-time) ----------
    let mut rng = thread_rng();
    // Empty-witness version of the circuit defines the constraint system
    let blank_circuit = AddCircuit::<Fr> {
        a: None,
        b: None,
        c: None,
    };
    let params = generate_random_parameters::<Bn254, _, _>(blank_circuit, &mut rng)?;
    let ProvingKey { vk, .. } = &params;

    // ---------- Proving ----------
    let proof = create_random_proof(circuit, &params, &mut rng)?;

    // ---------- Verification ----------
    // Groth16 wants *only* the public inputs (c) as field elements
    let public_inputs = vec![Fr::from(6u64)];
    let pvk = prepare_verifying_key(vk);

    let ok = verify_proof(&pvk, &proof, &public_inputs)?;
    println!("Proof is valid? {ok}");
    Ok(())
}
