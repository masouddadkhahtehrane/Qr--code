use std::io;
use qrcode::QrCode;
use qrcode::render::unicode;

fn main() {
    println!("Enter the text to generate QR Code:");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let input = input.trim(); // Remove extra characters like newline

    match QrCode::new(input) {
        Ok(code) => {
            let qr_display = code.render::<unicode::Dense1x2>().build();
            println!("\nQR Code:\n");
            println!("{}", qr_display);
        }
        Err(e) => {
            println!("Error generating QR Code: {}", e);
        }
    }
}
