use std::vec;

use openssl::{
    rsa::{
        Rsa,
        Padding,
    },
};

#[derive(Debug)]
pub struct Crypto {
    buf        : Vec<u8>,
    passphrase : Option<Vec<u8>>,
    private_key: Option<Vec<u8>>,
    public_key : Option<Vec<u8>>
}

impl Crypto {
    fn check_fields(&self) {
    }

    pub fn new() -> Self {
        Self {
            buf: vec![],
            passphrase: None,
            private_key: None,
            public_key: None
        }
    }

    pub fn new_with_keys(private: Option<String>, public: Option<String>) -> Self {
        Self {
            buf: vec![],
            private_key: if private.is_some() { Some(private.unwrap().into()) } else { None },
            public_key: if public.is_some() { Some(public.unwrap().into()) } else { None },
            passphrase: None
        }
    }

    pub fn set_passphrase<T>(&mut self, passphrase: T) 
    where 
    T: AsRef<str>
    {
        let passwd = passphrase.as_ref();

        self.passphrase = Some(passwd.as_bytes().into());
    }

    pub fn encrypt<T>(&mut self, data: T) -> Vec<u8>
    where 
        T: AsRef<str> 
    {
        let get_rsa = Rsa::public_key_from_pem(self.public_key.as_ref().unwrap());

        match get_rsa {
            Ok(rsa) => {
                self.buf = vec![0u8; rsa.size() as usize];

                rsa.public_encrypt(
                    data.as_ref().as_bytes(),
                    &mut self.buf,
                    Padding::PKCS1
                ).expect(format!("coudn't not encrypt ! at {}", line!()).as_str());

                self.buf.clone()
            }, 

            Err(e) => panic!("could not parse rsa file {:#?}", e)
        }
    }


    pub fn decrypt(&mut self, data: &[u8]) -> Vec<u8>
    {
        let private_key = self.private_key.as_ref().unwrap();

        let get_rsa = if let Some(v) = &self.passphrase {
            Rsa::private_key_from_pem_passphrase(private_key, &v)
        } else {
            Rsa::private_key_from_pem(private_key)
        };

        match get_rsa {
            Ok(rsa) => {
                self.buf = vec![0u8; rsa.size() as usize];

                rsa.private_decrypt(
                    data,
                    &mut self.buf,
                    Padding::PKCS1
                ).expect(format!("could not decrypt ! at {}", line!()).as_str());

                self.buf.clone()
            },

            Err(e) => panic!("could not parse rsa file {:#?}", e)
        }
    }
}