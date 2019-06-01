use beautiful_qrcode_generator_beemly::*;

fn main() {
    qrcode_generate("B").save("target/qrcode0.png").unwrap();
    qrcode_generate("https://beemly/")
        .save("target/qrcode1.png")
        .unwrap();
    qrcode_generate("Ceci est un message plus long")
        .save("target/qrcode2.png")
        .unwrap();
}
