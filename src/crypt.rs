use argon2::{self, Config, ThreadMode, Variant, Version};

const PASSWORD_LENGTH: u32 = 16;

fn build_argon2_config<'a>(secret: &'a [u8], additional_data: &'a [u8]) -> Config {
    return Config {
        variant: Variant::Argon2id,
        version: Version::Version13,
        mem_cost: 4096,
        time_cost: 3,
        lanes: 2,
        thread_mode: ThreadMode::Parallel,
        secret,
        ad: additional_data,
        hash_length: PASSWORD_LENGTH,
    };
}

fn extract_hash(hash_encoded: &str) -> String {
    return match hash_encoded.split('$').nth(5) {
        Some(value) => value.to_owned(),
        None => panic!("Sorry! An error occurred during password generation."),
    };
}

pub fn derive_password<'a>(
    master_password: &'a str,
    resource_identifier: &'a str,
    secret: &'a str,
    additional_data: &'a str,
) -> String {
    let config = build_argon2_config(secret.as_bytes(), additional_data.as_bytes());
    let hash_encoded = argon2::hash_encoded(
        resource_identifier.as_bytes(),
        master_password.as_bytes(),
        &config,
    )
    .unwrap();
    return extract_hash(&hash_encoded);
}
