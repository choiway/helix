use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use trade_signal::TradeSignal;
use writer;
use chromosome::Chromosome;
use repo;
use std::fs;

/// Write signals to disk
///
pub fn call(signals: &BTreeMap<String, TradeSignal>, chromosome: &Chromosome) {
    info!("writing id: {} to disk", chromosome.id);
    let filename = format!("/tmp/{}.txt", chromosome.id);
    let mut f = File::create(&filename).expect("Unable to create file");
    for signal in signals {
        let s = signal.1;
        write!(
            f,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            s.chromosome_id,
            s.ts,
            writer::fmt_vec_string(s.strategies.clone()),
            writer::fmt_vec_dna(s.signals.clone()),
            s.target_ticker,
            s.hard_signal,
            s.generation,
            s.ret,
            s.pnl
        ).unwrap();
    }
    repo::trade_signals::copy(&filename);
    fs::remove_file(filename).unwrap();
}

