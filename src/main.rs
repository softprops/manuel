extern crate hoedown;
extern crate manuel;

use hoedown::Markdown;
use hoedown::renderer::Render;
use manuel::Roff;

pub fn main() {
  let doc = Markdown::new("% manuel(1) markdown to roff
% Doug Tangren
% July 2015
# Name

This is **manuel**

# h1

## h2

### h3

some _emphasis_ __required__ *star*

this is a `code span`.

hi again

```
fenced code
is nice
```

    code is here
    and here
    in a block

I'm not code *fenced* or blocked

  ");
  //let mut html = Html::new(html::Flags::empty(), 0);
  //println!("{}", html.render(&doc).to_str().unwrap());
  println!("{}", Roff.render(&doc).to_str().unwrap());
}
