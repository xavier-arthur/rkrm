use openssl::{
    rsa::{
        Rsa,
        Padding,
    },
};

use std::fs::{
    read_to_string
};

pub fn encrypt<T>(data: T, pub_key: &str) -> Vec<u8>
where 
    T: AsRef<str> 
{
    let rsa = Rsa::public_key_from_pem(pub_key.as_bytes()).unwrap();
    let mut buf = vec![0u8; rsa.size() as usize];
    let encrypt = rsa.public_encrypt(
        data.as_ref().as_bytes(),
        &mut buf,
        Padding::PKCS1
    );

    match encrypt {
        Ok(_)  => buf,
        Err(e) => panic!("while encrypting {:#?}", e)
    }
}

pub fn decrypt(data: &[u8], private_key: &str, passphrase: Option<&str>) -> Vec<u8>
where 
    T: AsRef<str>
{
    let private_key = private_key.as_bytes();

    let get_rsa = if let Some(v) = passphrase {
        let passwd = v.as_bytes();

        Rsa::private_key_from_pem_passphrase(private_key, passwd)
    } else {
        Rsa::private_key_from_pem(private_key)
    };

    match get_rsa {
        Ok(rsa) => {
            let mut buf = vec![0u8; rsa.size() as usize];
            let decrypted = rsa.private_decrypt(
                data,
                &mut buf,
                Padding::PKCS1
            ).expect(format!("coudn't not decrypt ! at {}", line!()).as_str());

            buf.into_iter().filter(|v| *v != 0).collect()
        },

        Err(e) => panic!("could not parse rsa file {:#?}", e)
    }
}