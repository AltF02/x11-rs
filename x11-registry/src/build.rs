// This software is available under the terms of the MIT license.

pub mod lex;
pub mod parse;

use std::env;
use std::fs::File;
use std::path::PathBuf;
use std::io::{Read, Write};
use std::str;

fn writety (w: &mut File, ty: &parse::Type) {
    match *ty {
        parse::Type::Ident(id) => write!(w, "Type::Ident(\"{}\")", str::from_utf8(id).unwrap()).unwrap(),

        parse::Type::ConstPtr(ref inner) => {
            write!(w, "Type::ConstPtr(&").unwrap();
            writety(w, inner);
            write!(w, ")").unwrap();
        },

        parse::Type::MutPtr(ref inner) => {
            write!(w, "Type::MutPtr(&").unwrap();
            writety(w, inner);
            write!(w, ")").unwrap();
        },
    }
}

fn gen (name: &str, namecaps: &str) {
    let fullname = format!("{}.registry", name);

    let mfdir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let inpath = mfdir.join("registry").join(&fullname);
    let mut infile = File::open(&inpath).expect("can't open input file");
    let mut insrc = String::new();
    infile.read_to_string(&mut insrc).expect("can't read input file");

    let outdir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR not set"));
    let outpath = outdir.join(format!("{}.rs", name));
    let mut outfile = File::create(&outpath).expect("can't open output file");

    let lexer = lex::Lexer::new(fullname.as_str(), insrc.as_str().as_bytes());
    let parser = parse::Parser::new(lexer);
    let fns: Vec<parse::Fn> = parser.collect();

    writeln!(outfile, "pub static {}: Registry = Registry {{ fns: &[", namecaps).unwrap();

    for f in fns.iter() {
        write!(outfile, "Fn {{ name: \"{}\", return_type: ", str::from_utf8(f.name).unwrap()).unwrap();

        match f.return_ty {
            None => write!(outfile, "None").unwrap(),
            Some(ref t) => {
                write!(outfile, "Some(").unwrap();
                writety(&mut outfile, t);
                write!(outfile, ")").unwrap();
            },
        }

        write!(outfile, ", params: &[").unwrap();

        for t in f.params.iter() {
            writety(&mut outfile, t);
            write!(outfile, ", ").unwrap();
        }

        writeln!(outfile, "] }},").unwrap();
    }

    writeln!(outfile, "] }};").unwrap();
}

fn main () {
    if cfg!(feature = "libx11") { gen("libx11", "LIBX11"); }
}
