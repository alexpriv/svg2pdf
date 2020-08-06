// Copyright (c) 2020 Alex Pedley, all rights reserved. ISC license.
use env_logger;
use log::{debug, error};
use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use structopt::StructOpt;
use wkhtmltopdf::{Orientation, PdfApplication, Size};

macro_rules! fatal {
    ( $( $x:expr ),* ) => {{
        error!($($x),*);
        std::process::exit(1);
    }};
}

#[derive(StructOpt)]
#[structopt(
    name = env!("CARGO_PKG_NAME"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION")
)]
struct Cli {
    /// Enables verbose logging: -v = debug, -vv = trace.
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

    /// An array of SVG locations. These will each have their own page.
    #[structopt(parse(from_os_str))]
    files: Vec<std::path::PathBuf>,
    /// Sets the outfile of the PDF. If not specified, the output is written to stdout.
    #[structopt(short, long, parse(from_os_str))]
    out: Option<std::path::PathBuf>,

    /// Sets the margin of the PDF in inches or millimetres. Pass the --metric flag to use mm.
    #[structopt(short, long, default_value = "0")]
    margin: u32,
    /// Makes the PDF landscape.
    #[structopt(short, long)]
    landscape: bool,
    /// Use metric units for margins, etc.
    #[structopt(long)]
    metric: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Cli = Cli::from_args();

    env::set_var(
        "RUST_LOG",
        if args.verbose == 0 {
            "info"
        } else if args.verbose == 1 {
            "debug"
        } else {
            "trace"
        },
    );
    env_logger::init();

    if args.files.len() == 0 {
        fatal!("At least one file must be passed");
    }

    let mut content = String::new();
    for (i, file) in args.files.iter().enumerate() {
        debug!("Reading file {}", i);
        match File::open(file) {
            Ok(f) => {
                debug!("Proceeding for {}", i);
                content.push_str(r#"<div style="height:100vh;">"#);

                let mut reader = BufReader::new(f);
                reader.read_to_string(&mut content)?;

                content.push_str(r#"</div>"#);
                debug!("{} done!", i);
            }
            Err(e) => fatal!("{:?}: {}", file, e),
        }
    }

    debug!("Building PDF");
    let mut pdf_app = PdfApplication::new()?;
    let mut out = pdf_app
        .builder()
        .orientation(if args.landscape {
            Orientation::Landscape
        } else {
            Orientation::Portrait
        })
        .margin(if args.metric {
            Size::Millimeters(args.margin)
        } else {
            Size::Inches(args.margin)
        })
        .build_from_html(content)?;

    if let Some(out_file) = args.out {
        debug!("Writing to {:?}", out);
        out.save(out_file)?;
    } else {
        let mut res = Vec::new();
        out.read_to_end(&mut res)?;
        std::io::stdout().write(&res)?;
    }
    Ok(())
}
