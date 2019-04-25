use rust_design_qrcode::*;

fn main() {
    let matrix = qrcode_matrix("https://beemly.app/scan/");
    for line in matrix {
        for block in line {
            if block {
                print!("██");
            } else {
                print!("  ");
            }
        }
        print!("\n");
    }
}
