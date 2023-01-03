use std::fs::File;
use std::io::prelude::*;

use crate::airtable::AirtableRecord;

#[derive(Debug)]
struct RSA {
    name: String,
    slug: String,
    address: String,
    city: String,
    telephone: String,
    email: String,
    website: String,
    hospitalization_type: Vec<String>,
    conventions: Vec<String>,
}

impl RSA {
    fn from_record(row: AirtableRecord) -> Self {
        Self {
            name: row.string("Nome").unwrap(),
            slug: row.string("Slug").unwrap_or("".to_string()),
            address: row.string("Indirizzo").unwrap_or("".to_string()),
            city: row.string("Città").unwrap_or("".to_string()),
            telephone: row.string("Telefono").unwrap_or("".to_string()),
            email: row.string("Email").unwrap_or("".to_string()),
            website: row.string("Sito Web").unwrap_or("".to_string()),
            hospitalization_type: row.records("Tipo"),
            conventions: row.records("Convenzioni"),
        }
    }

    fn is_nap(self: &Self) -> bool {
        self.hospitalization_type.contains(&"NAP".to_string())
    }

    fn is_nat(self: &Self) -> bool {
        self.hospitalization_type.contains(&"NAT".to_string())
    }

    fn is_prima_fascia(self: &Self) -> bool {
        self.hospitalization_type
            .contains(&"PRIMA FASCIA".to_string())
    }

    fn is_nucleo_alzhaimer(self: &Self) -> bool {
        self.hospitalization_type
            .contains(&"NUCLEO ALZHEIMER".to_string())
    }

    fn to_markdown(&self) -> String {
        let title = format!("{}", self.name);
        let city = format!("{}", self.city);
        let is_nap = self.is_nap();
        let is_nat = self.is_nat();
        let is_prima_fascia = self.is_prima_fascia();
        let is_nucleo_alzhaimer = self.is_nucleo_alzhaimer();
        let email = self.email.clone();
        let telephone = self.telephone.clone();
        let website = self.website.clone();
        let address = self.address.clone();
        let municipaly_convention = self.conventions.contains(&"Comunale".to_string());
        let asl_convention = self.conventions.contains(&"ASL".to_string());
        format!(
            r#"+++
title = "{title}"
[extra]
city = "{city}"
is_nap = {is_nap}
is_nat = {is_nat}
is_prima_fascia = {is_prima_fascia}
is_nucleo_alzhaimer = {is_nucleo_alzhaimer}
asl_convention = {asl_convention}
municipaly_convention = {municipaly_convention}
email = "{email}"
telephone = "{telephone}"
website = "{website}"
address = "{address}"
+++
"#
        )
    }

    fn slug(&self) -> String {
        self.slug.clone()
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

pub async fn generate_all() {
    let airtable = crate::airtable::Airtable::new(
        &std::env::var("AIRTABLE_TOKEN").expect("AIRTABLE_TOKEN must be set"),
    );
    let records = airtable.records("app0PapFh5s0YJGkj", "RSA").await;

    // for entry in std::fs::read_dir("tmp/my_items").unwrap() {
    //     entry.unwrap().file_name()
    //     std::fs::remove_file(entry);
    // }

    // let mut rsas = Vec::new();
    for record in records {
        let rsa = RSA::from_record(record);
        let mut file = File::create(format!("./website/content/{}.md", rsa.slug())).unwrap();
        file.write_all(rsa.to_markdown().as_bytes()).unwrap();
        println!("{:?}", rsa.name());
    }
}
