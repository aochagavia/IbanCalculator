mod settings;

use settings::Settings;

fn main() {
    let settings = Settings::from_args();
    println!("{:?}", settings);
    println!("{}", m_proef(274856190, 11));
}

fn m_proef(test: u32, modulo: u32) -> bool {
    let mut rest: u32 = test;
    let mut counter: u32 = 0;
    let mut index: u32 = 1;
    while rest != 0 {
        counter += (rest % 10) * index;
        rest /= 10;
        index += 1;
    }
    (counter % modulo) == 0
}
