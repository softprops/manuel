extern crate hoedown;

use std::io::Write;
use hoedown::Buffer;
use hoedown::renderer::{ self, Render };

pub struct Roff;

impl Render for Roff {
  fn code_block(&mut self, output: &mut Buffer, text: &Buffer, lang: &Buffer) {
    output.write(b"\n.PP\n.RS\n\n.nf\n").unwrap();
    output.write(text).unwrap(); // todo: esc
    output.write(b"\n.fi\n.RE\n").unwrap();
  }

  fn quote_block(&mut self, output: &mut Buffer, content: &Buffer) {
    output.write(b"\n.PP\n.RS\n").unwrap();
    output.write(content).unwrap();
    output.write(b"\n.RE\n").unwrap();
  }

  fn header(&mut self, output: &mut Buffer, content: &Buffer, level: i32) {
    if output.is_empty() {
      output.write(b".TH ").unwrap();
    }
    match level {
      1 => output.write(b"\n\n.SH ").unwrap(),
      2 => output.write(b"\n.SH ").unwrap(),
      _ => output.write(b"\n.SS ").unwrap()
    };
    output.write(content).unwrap();
  }

  fn horizontal_rule(&mut self, output: &mut Buffer) {
    output.write(b"\n.ti 0\n\\l'\\n(.lu'\n").unwrap();
  }

  fn list(&mut self, output: &mut Buffer, content: &Buffer, flags: renderer::list::List) {
    output.write(b".IP ").unwrap();
    output.write(b"\n").unwrap();
    // fixme: is this right?
    output.write(content).unwrap();
  }

  fn list_item(&mut self, output: &mut Buffer, content: &Buffer, flags: renderer::list::List) {
    output.write(b"\n\\item ").unwrap();
    output.write(content).unwrap();
  }

  fn paragraph(&mut self, output: &mut Buffer, content: &Buffer) {
    let empty = output.is_empty();
    output.write(b"\n.PP\n").unwrap();
    output.write(content).unwrap();
    if empty {
      output.write(b"\n").unwrap();
    }
  }

  fn table(&mut self, output: &mut Buffer, content: &Buffer) {
    output.write(b".TS\nallbox;\n").unwrap();
    output.write(content).unwrap();
    output.write(b"\n.TE\n").unwrap();
  }

  fn table_header(&mut self, output: &mut Buffer, content: &Buffer) {
    if !output.is_empty() {
      output.write(b" ").unwrap();
    }
    output.write(content).unwrap();
    output.write(b" ").unwrap();
  }

  fn table_body(&mut self, output: &mut Buffer, content: &Buffer) {
    output.write(content).unwrap();
  }

  fn table_row(&mut self, output: &mut Buffer, content: &Buffer) {
    if !output.is_empty() {
      output.write(b"\n").unwrap();
    }
    output.write(content).unwrap();
    output.write(b"\n").unwrap();
  }

  fn table_cell(&mut self, output: &mut Buffer, content: &Buffer, flags: renderer::Table) {
    if !output.is_empty() {
      output.write(b"\t").unwrap();
    }
    output.write(content).unwrap();
    output.write(b"\t").unwrap();
  }

  fn footnotes(&mut self, output: &mut Buffer, content: &Buffer) {
    output.write(b"MISSING FOOTNOTES HANDLER\n").unwrap();
  }

  fn footnote_definition(&mut self, output: &mut Buffer, content: &Buffer, num: u32) {
    output.write(b"MISSING FOOTNOTE_DEFINITION HANDLER\n").unwrap();
  }

  fn html_block(&mut self, output: &mut Buffer, text: &Buffer) {
    output.write(text).unwrap();
  }

  fn autolink(&mut self, output: &mut Buffer, link: &Buffer, ty: renderer::AutoLink) -> bool {
    output.write(b"\n\\[la]").unwrap();
    output.write(link).unwrap(); // fixme: may need esc
    output.write(b"\\[ra]").unwrap();
    true
  }

  fn code_span(&mut self, output: &mut Buffer, text: &Buffer) -> bool {
    output.write(b"\\fB\\fC").unwrap();
    output.write(text).unwrap();
    output.write(b"\\fR").unwrap();
    true
  }

  fn double_emphasis(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
    output.write(b"\\fB").unwrap();
    output.write(content).unwrap();
    output.write(b"\\fP").unwrap();
    true
  }

  fn emphasis(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
    output.write(b"\\fI").unwrap();
    output.write(content).unwrap();
    output.write(b"\\fP").unwrap();
    true
  }

  fn underline(&mut self, ob: &mut Buffer, content: &Buffer) -> bool {
    println!("no underline..");
    false
  }

  fn highlight(&mut self, ob: &mut Buffer, content: &Buffer) -> bool {
    println!("no highlight...");
    false
  }

  fn quote_span(&mut self, ob: &mut Buffer, content: &Buffer) -> bool {
    println!("no quot");
    false
  }

  fn image(&mut self, ob: &mut Buffer, link: &Buffer, title: &Buffer, alt: &Buffer) -> bool {
    false
  }

  fn line_break(&mut self, output: &mut Buffer) -> bool {
    output.write(b"\n.br\n").unwrap();
    true
  }

  fn link(&mut self, output: &mut Buffer, content: &Buffer, link: &Buffer, title: &Buffer) -> bool {
    output.write(b"\n\\[la]").unwrap();
    output.write(content).unwrap();
    output.write(b"\\[ra]").unwrap();
    true
  }

  fn triple_emphasis(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
    output.write(b"\\s+2").unwrap();
    output.write(content).unwrap();
    output.write(b"\\s-2").unwrap();
    true
  }

  fn strikethrough(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
    false
  }

  fn superscript(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
    false
  }

  fn footnote_reference(&mut self, output: &mut Buffer, num: u32) -> bool {
    false
  }

  fn math(&mut self, output: &mut Buffer, text: &Buffer, displaymode: i32) -> bool {
    false
  }

  fn html_span(&mut self, output: &mut Buffer, text: &Buffer) -> bool {
    output.write(text).unwrap();
    true
  }

  fn entity(&mut self, output: &mut Buffer, text: &Buffer) {
    output.write(text).unwrap();
  }

  fn normal_text(&mut self, output: &mut Buffer, text: &Buffer) {
    output.write(text).unwrap();
  }

  fn before_render(&mut self, output: &mut Buffer, inline_render: bool) {
    ()
  }

  fn after_render(&mut self, output: &mut Buffer, inline_render: bool) {
    ()
  }
}

#[test]
fn it_works() {
}
