use rust_design_qrcode::*;

fn main() {
    qrcode_generate("Hello World !").save("target/qrcode.png");
}
