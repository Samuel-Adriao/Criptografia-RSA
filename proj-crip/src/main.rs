use num_bigint::{BigInt, BigUint, ToBigInt, ToBigUint};
use num_traits::{One, Zero};
use num_primes::Generator; // para gerar primos grandes aleatórios

/// Calcula o máximo divisor comum (método de Euclides)
fn gcd(a: &BigUint, b: &BigUint) -> BigUint {
    if b.is_zero() {
        a.clone()
    } else {
        gcd(b, &(a % b))
    }
}

/// Calcula o inverso modular usando o algoritmo de Euclides estendido
fn modinv(a: &BigInt, m: &BigInt) -> Option<BigInt> {
    let (mut t, mut new_t) = (BigInt::zero(), BigInt::one());
    let (mut r, mut new_r) = (m.clone(), a.clone());

    while !new_r.is_zero() {
        let quotient = &r / &new_r;
        let temp_t = &t - &quotient * &new_t;
        let temp_r = &r - &quotient * &new_r;
        t = new_t;
        new_t = temp_t;
        r = new_r;
        new_r = temp_r;
    }

    if r > BigInt::one() {
        return None; // sem inverso modular
    }

    if t < BigInt::zero() {
        t += m;
    }

    Some(t)
}

/// Gera chaves RSA
fn generate_keys(bits: usize) -> (BigUint, BigUint, BigUint) {
    // Gera dois primos grandes p e q
    let p = Generator::new_prime(bits / 2);
    let q = Generator::new_prime(bits / 2);

    let n = &p * &q;
    let phi = (&p - BigUint::one()) * (&q - BigUint::one());

    // Escolhe e (número primo comum usado em RSA)
    let e = BigUint::from(65537u32);

    // Calcula d = e^-1 mod φ(n)
    let e_bigint = e.to_bigint().unwrap();
    let phi_bigint = phi.to_bigint().unwrap();

    let d = modinv(&e_bigint, &phi_bigint)
        .expect("Não foi possível calcular o inverso modular")
        .to_biguint()
        .unwrap();

    (n, e, d)
}

/// Criptografa uma mensagem numérica m
fn encrypt(m: &BigUint, e: &BigUint, n: &BigUint) -> BigUint {
    m.modpow(e, n)
}

/// Descriptografa uma mensagem numérica c
fn decrypt(c: &BigUint, d: &BigUint, n: &BigUint) -> BigUint {
    c.modpow(d, n)
}

/// Converte texto para número e vice-versa
fn text_to_biguint(text: &str) -> BigUint {
    BigUint::from_bytes_be(text.as_bytes())
}

fn biguint_to_text(num: &BigUint) -> String {
    String::from_utf8(num.to_bytes_be()).unwrap_or_else(|_| "<texto inválido>".into())
}

fn main() {
    // === Geração de chaves ===
    let (n, e, d) = generate_keys(512); // 512 bits para demonstração
    println!("🔑 Chave pública (n, e):\n({}, {})", n, e);
    println!("🔒 Chave privada (d):\n{}", d);

    // === Mensagem original ===
    let mensagem = "RSA em Rust!";
    println!("\nMensagem original: {}", mensagem);

    let m = text_to_biguint(mensagem);

    // === Criptografia ===
    let c = encrypt(&m, &e, &n);
    println!("Mensagem criptografada: {}", c);

    // === Descriptografia ===
    let dec = decrypt(&c, &d, &n);
    let texto_decifrado = biguint_to_text(&dec);
    println!("Mensagem decifrada: {}", texto_decifrado);
}
