use sha2::{Digest, Sha256};
use std::{
    fmt::Debug,
    fs::OpenOptions,
    io::Write,
    time::{SystemTime, UNIX_EPOCH},
};

const BITE_TO_BYTES: usize = 8;

#[derive(Debug)]
pub struct User {
    _name: String,
    salt: Option<[u8; 256 / BITE_TO_BYTES]>,
    password_hash: Option<[u8; 256 / BITE_TO_BYTES]>,
}

impl User {
    pub fn new(name: &str) -> Self {
        Self {
            _name: name.to_owned(),
            salt: None,
            password_hash: None,
        }
    }

    fn generate_salt(self: &mut Self) -> &Self {
        let time = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => n.as_secs().to_be_bytes(),
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        };

        let mut hasher = Sha256::new();
        hasher.update(time);
        self.salt = Some(hasher.finalize().into());

        self
    }

    pub fn set_password(self: &mut Self, password: &str) -> &Self {
        self.generate_salt();

        let mut password = password.as_bytes().to_vec();
        password.append(&mut self.salt.clone().unwrap().to_vec());

        let mut hasher = Sha256::new();
        hasher.update(password);
        self.password_hash = Some(hasher.finalize().into());

        self
    }

    pub fn check_password(self: &mut Self, password: &str) -> Result<(), PasswordErr> {
        if self.salt == None || self.password_hash == None {
            return Err(PasswordErr::NotSet);
        };

        let mut password = password.as_bytes().to_vec();
        password.append(&mut self.salt.clone().unwrap().to_vec());

        let mut hasher = Sha256::new();
        hasher.update(password);
        let input_password_hash: [u8; 256 / BITE_TO_BYTES] = hasher.finalize().into();

        match input_password_hash == self.password_hash.unwrap() {
            true => Ok(()),
            false => Err(PasswordErr::Wrong),
        }
    }

    pub fn log_to_file(self: &Self, filename: &str) {
        let mut file = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(filename)
            .unwrap();
        file.write_all(self._name.clone().as_bytes()).unwrap();
        file.write_all(b" ").unwrap();
        file.write_all(&self.salt.unwrap().clone()).unwrap();
        file.write_all(b" ").unwrap();
        file.write_all(&self.password_hash.unwrap().clone())
            .unwrap();
        file.write_all(b" \n").unwrap();
    }
}

#[derive(Debug)]
pub enum PasswordErr {
    NotSet,
    Wrong,
}
