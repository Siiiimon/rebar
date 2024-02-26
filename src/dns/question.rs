pub fn address_to_question(address: &String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let parts = address.split('.');
    let mut labels: Vec<u8> = Vec::new();
    for part in parts {
        labels.push(part.len() as u8);
        let mut bytes = part.to_string().into_bytes();
        labels.append(&mut bytes);
    }

    labels.push(0 as u8);

    labels.push(0 as u8);
    labels.push(1 as u8);
    labels.push(0 as u8);
    labels.push(1 as u8);

    Ok(labels)
}
