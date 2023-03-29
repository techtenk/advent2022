
fn get_input() -> Vec<char> {
    include_bytes!("input.txt").to_vec().iter().map(|c| char::from(*c)).collect()
}

fn unique(last_signals: &[char]) -> bool {
    let mut deduped = last_signals.into_iter().copied().collect::<Vec<char>>();
    deduped.sort();
    deduped.dedup();
    if deduped.len() == last_signals.len() {
        return true;
    }
    false
}

enum MarkerType {
    StartPacket,
    StartMessage
}

fn run(marker: MarkerType) -> u32 {
    let signal = get_input();
    let sig_size = match marker { MarkerType::StartPacket => 4, MarkerType::StartMessage => 14 };
    let mut i = sig_size;
    let mut last_signals = &signal[..i];

    while !unique(last_signals) {
        i += 1;
        if i < signal.len() {
            last_signals = &signal[i-sig_size..i];
        } else {
            panic!("Got to the end of the signal with no markers!");
        }
        
    }
    i as u32
}

pub fn run_part1() {
    println!("Day 6 part 1");

    println!("Marker found at {}", run(MarkerType::StartPacket));
}

pub fn run_part2() {
    println!("Day 6 part 2");

    println!("Marker found at {}", run(MarkerType::StartMessage));

}