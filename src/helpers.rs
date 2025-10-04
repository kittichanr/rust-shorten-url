const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"; // base62

pub fn base62_encode(number: u64) -> String {
    let length = ALPHABET.len() as u64;
    let mut encoded = String::new();

    let mut num = number;
    while num > 0 {
        let remainder = (num % length) as usize;
        encoded.push(ALPHABET.chars().nth(remainder).unwrap());
        num /= length;
    }

    encoded
}

pub fn base62_decode(encoded_string: &str) -> Result<u64, String> {
    let length = ALPHABET.len() as u64;
    let mut number: u64 = 0;

    for (i, symbol) in encoded_string.chars().enumerate() {
        if let Some(pos) = ALPHABET.find(symbol) {
            let pos = pos as u64;
            number += pos * length.pow(i as u32);
        } else {
            return Err(format!("Cannot find symbol '{}' in alphabet", symbol));
        }
    }

    Ok(number)
}

pub fn enforce_http(url: &str) -> String {
    if !url.starts_with("http") {
        format!("http://{}", url)
    } else {
        url.to_string()
    }
}

pub fn remove_domain_error(url: &str, domain: &str) -> bool {
    let url = url.to_lowercase();
    if url == domain {
        return false;
    }

    let mut new_url = url.to_string();
    new_url = new_url.replace("http://", "");
    new_url = new_url.replace("https://", "");
    new_url = new_url.replace("www.", "");

    if let Some(pos) = new_url.find("/") {
        new_url = new_url[pos..].to_string();
    }

    new_url != domain
}
