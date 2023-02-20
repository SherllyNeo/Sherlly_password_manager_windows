use tindercrypt::cryptors::RingCryptor;

//takes in the plain text and the passphrase, will return bytes as encrypyed
pub fn encrypt_text(text: String,passphrase: String) -> Vec<u8> {
    let text_b = text.as_bytes();
    let pass_b = passphrase.as_bytes();
    let cryptor = RingCryptor::new();

    let ciphertext = cryptor.seal_with_passphrase(pass_b, text_b).expect("unable to encrypt");

    ciphertext
}
