mod cli;
use qrcode::{QrCode, render::unicode};
use std::env::var;
use rand::{thread_rng, Rng, distributions::{Alphanumeric, DistString}};
use clap::Parser;

//otpauth://totp/opensuse@localhost.localdomain?secret=QOO6Z4FRLA4PRPQODJGOWZTBEI&issuer=localhost.localdomain

fn main() {
    let args = cli::Args::parse();

    let duration = match args.duration {
        Some(dur) => dur,
        None => 17,
    };
    
    let user = match args.user {
        Some(user) => user,
        None => var("USER").unwrap_or("user".to_string()),
    };

    let host = args.host.unwrap_or("localhost".to_string());
    let domain = args.domain.unwrap_or("localdomain".to_string());

    let mut rng = thread_rng();
    
    let secret = Alphanumeric.sample_string(&mut rng, 30);

    let mut out = format!(r#"{secret}
" RATE_LIMIT 3 30
" WINDOW_SIZE {}
" DISALLOW_REUSE
" TOTP_AUTH"#, duration);
    
    for _ in 0..9 {
        let scratch_code = rng.gen_range(10000000..100000000);
        out.push_str(&format!("\n{scratch_code}"));
    }

    let url = format!("otpauth://totp/{user}@{host}.{domain}?secret={secret}&issuer={host}.{domain}");
    
    let qr = QrCode::new(&url).unwrap();


    println!("{}", qr.render::<unicode::Dense1x2>().build());

    

    println!("{}",out);
}
