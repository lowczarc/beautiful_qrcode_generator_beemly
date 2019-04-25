use beautiful_qrcode_generator_beemly::*;

fn main() {
    qrcode_generate("Hello World !").save("target/qrcode.png");
}
