use chalk_rs::Chalk;
use debugoff::{self, multi_ptraceme_or_die};
use figlet_rs::FIGfont;
use obfstr::obfstr;
use spinners::{Spinner, Spinners};

const CYBERMEDIUM_FONT_BYTES: &[u8] = include_bytes!("../assets/cybermedium.flf");

pub fn print_banner() {
    let mut chalk = Chalk::new();
    multi_ptraceme_or_die();
    let font_str =
        std::str::from_utf8(CYBERMEDIUM_FONT_BYTES).expect(obfstr!("Invalid UTF-8 in font file."));
    let custom_font = FIGfont::from_content(font_str).expect(obfstr!("Invalid font file format."));

    let figure = custom_font.convert(obfstr!("Domain Grabber"));

    chalk.green().println(&figure.unwrap());
    chalk.cyan().underline().println(&obfstr!(
        "Grab all registered domains easily! | made with 💖 by @decryptable using Rust"
    ));
}

pub fn show_loading() -> Spinner {
    let mut chalk = Chalk::new();
    let sp = Spinner::new(Spinners::Dots9, chalk.gray().string(&obfstr!(" Scraping domains...")));

    return sp;
}
