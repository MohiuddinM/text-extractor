#![feature(libc)]

extern crate regex;
extern crate time;
extern crate html5ever;
extern crate tendril;
extern crate libc;
extern crate rayon;
#[macro_use]
extern crate html5ever_atoms;

use html5ever::{ParseOpts, parse_document};
use html5ever::rcdom::{Doctype, Text, Comment, Element, RcDom, Handle, NodeEnum, Node};
use tendril::TendrilSink;

use std::str::FromStr;
use std::error::Error;
// use std::fs::File;
// use std::io::prelude::*;
// use std::path::Path;
use regex::Regex;

use time::precise_time_ns;

use libc::c_char;
use std::ffi::CStr;
use std::ffi::CString;
use std::str;

// use std::mem;

fn get_text(input: &str) -> Vec<String> {
    let dom = parse_document(RcDom::default(), ParseOpts::default())
        .from_utf8()
        .one(input.as_bytes());

    walk(dom.document)
}

fn walk(handle: Handle) -> Vec<String> {
    let node = handle.borrow();
    match node.node {
        Text(ref s) => {
            let mut text: Vec<String> = vec![];
            for split_text in (&**s).split('|') {
                if !split_text.contains('\t') && split_text.split(' ').count() > 3 {
                    text.push(split_text.trim().replace("\n", ""));
                }
            }
            for child in node.children.iter() {
                text.append(&mut walk(child.clone()));
            }
            return text;
        }

        Element(ref name, _, ref attrs) => {
            let mut text = vec![];
            match name.local.as_ref() {
                "script" | "style" | "meta" | "noscript" | "nav" | "iframe" | "img" | "link" |
                "header" | "footer" | "form" | "button" | "aside" | "span" => {}
                _ => {
                    for child in &node.children {
                        text.append(&mut walk(child.clone()));
                    }
                }
            };
            return text;
        }
        _ => {
            let mut text = vec![];
            for child in &node.children {
                text.append(&mut walk(child.clone()));
            }
            return text;
        }
    }
}

fn process(html: &str) -> String {
    let mut s = html.replace("<i>", "")
        .replace("</i>", "")
        .replace("<b>", "")
        .replace("</b>", "")
        .replace("<em>", "")
        .replace("</em>", "")
        .replace("    ", "\t");
    s = Regex::new(r"(<a.*?>(?P<t>.*?)</a>)").unwrap().replace_all(&s, "$t").into(); //replacing link with only text

    

    // let start = precise_time_ns();
    let texts = get_text(&s);
    // let stop = precise_time_ns();
    // let dur = Duration::nanoseconds((stop - start) as i64).num_microseconds().unwrap();
    // println!("{}us in inner loop", dur);

    let text = texts.join("\n");
    text
}



// fn string_to_static_str(s: String) -> &'static str {
// unsafe {
// let ret = mem::transmute(&s as &str);
// mem::forget(s);
// ret
// }
// }
//

#[no_mangle]
pub fn extract_text(html: *mut c_char) -> *mut c_char {
    let s = unsafe { CStr::from_ptr(html) };

    let s: String = match String::from_str(String::from_utf8_lossy(&mut s.to_bytes()).to_mut()) {
        Ok(res) => res,
        Err(why) => panic!("Couldn't convert response to UTF8. {}", why.description()),
    };

    let texts = process(&s);
    return CString::new(texts).unwrap().into_raw();
}

// fn main() {
// let path = Path::new("c:\\users\\muhammad\\desktop\\html\\doc2.htm");
// let display = path.display();
// let mut file = match File::open(&path) {
// Err(why) => panic!("couldn't open {}: {}", display, why.description()),
// Ok(file) => file,
// };
//
// let mut buffer = Vec::new();
// file.read_to_end(&mut buffer);
// let s: String = match String::from_str(String::from_utf8_lossy(&mut buffer).to_mut()) {
// Ok(res) => res,
// Err(why) => panic!("Couldn't convert response to UTF8. {}", why.description()),
// };
//
//
// let start = precise_time_ns();
// let texts = process(&s);
// let stop = precise_time_ns();
//
// let dur = Duration::nanoseconds((stop - start) as i64).num_milliseconds();
//
// println!("{}ms", dur);
//
// println!("Title: {}", texts.split("\n")[0]);
// println!("Words Retrieved {}", texts.split(' ').count());
// println!("{}", texts.1);
// See filtering from boilerpipe.net
//
// Click to share on Twitter (Opens in new window)
// Click to share on Facebook (Opens in new window)
// Click to share on Google+ (Opens in new window)
// }
//