use chrono::{Duration};
use openssl::x509::X509;
use openssl::pkey::PKey;
use openssl::stack::Stack;
use openssl::pkcs7::{Pkcs7, Pkcs7Flags};



pub fn build_login_ticket_request(service: &str) -> String {
    let now= chrono::Local::now();
    let generation_time =  now.format("%Y-%m-%dT%H:%M:%S%:z").to_string();
    let expiration_time = (now + Duration::minutes(10)).format("%Y-%m-%dT%H:%M:%S%:z").to_string();
    let unique_id = now.timestamp();

    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<loginTicketRequest version="1.0">
    <header>
        <uniqueId>{unique_id}</uniqueId>
        <generationTime>{generation_time}</generationTime>
        <expirationTime>{expiration_time}</expirationTime>
    </header>
    <service>{service}</service>
</loginTicketRequest>"#
    )
}


pub fn sign_login_ticket_request(xml: &str) -> String{
    let cert_bytes = std::fs::read("homologacion.crt").expect("no pude leer el certificado");
    let key_bytes = std::fs::read("privada.key").expect("no pude leer la clave privada");

    let cert = X509::from_pem(&cert_bytes).expect("certificado invalido");
    let pkey = PKey::private_key_from_pem(&key_bytes).expect("clave invalida");

    let extra_certs = Stack::new().expect("no pude crear el stack");

    let pkcs7 = Pkcs7::sign(&cert, &pkey, &extra_certs, xml.as_bytes(), Pkcs7Flags::BINARY)
    .expect("fallo al firmar");

    let der = pkcs7.to_der().expect("no pude convertir a DER");
    
    openssl::base64::encode_block(&der)
}