#![allow(clippy::disallowed_names, unused_variables, dead_code)]
use regex::Regex;

use std::io::{self, Read, Write};

fn transform_bold(input: &str) -> String {
    let re = Regex::new(r"\*\*(.*?)\*\*").unwrap();
    let transformed = re.replace_all(input, "*$1*").to_string();
    transformed
}

fn transform_double_underscore_bold(input: &str) -> String {
    let re = Regex::new(r"__(.*?)__").unwrap();
    let transformed = re.replace_all(input, "*$1*").to_string();
    transformed
}

fn transform_italic(text: &str) -> String {
    let re = Regex::new(r"\*(.*?)\*").unwrap();
    let replaced_text = re.replace_all(text, |caps: &regex::Captures<'_>| {
        if caps[1].is_empty() {
            caps[0].to_string()
        } else {
            format!("/{}/", &caps[1])
        }
    });
    replaced_text.into_owned()
}

fn transform_markdown_lists(text: &str) -> String {
    let re = Regex::new(r"(?m)^(\s*)([-*+])\s+(.*)$").unwrap();
    let transformed_text = re.replace_all(text, |caps: &regex::Captures<'_>| {
        let indentation = caps[1].len() / 4;
        let item_content = caps[3].trim();
        let dashes = "-".repeat(indentation);
        format!("{}{} {}", dashes, &caps[2], item_content)
    });
    transformed_text.into_owned()
}

fn transform_codeblocks(text: &str) -> String {
    let re = Regex::new(r"```(\w+)\n([\s\S]*?)\n```").unwrap();
    let transformed_text = re.replace_all(text, "@code $1\n$2\n@end");
    transformed_text.into_owned()
}

fn transform_headings(text: &str) -> String {
    let re = Regex::new(r"(?m)^(#{1,6})\s(.+)$").unwrap();
    let transformed_text = re.replace_all(text, |caps: &regex::Captures<'_>| {
        let header_level = caps.get(1).unwrap().as_str();
        let header_text = caps.get(2).unwrap().as_str();
        let asterisks = "*".repeat(header_level.len());
        format!("{} {}", asterisks, header_text)
    });
    transformed_text.into_owned()
}

fn transform_obsidian_links(text: &str) -> String {
    let re = Regex::new(r"\[\[([^|\]]+)\s*\|\s*([^]]+)\]\]").unwrap();
    let transformed_text = re.replace_all(text, |caps: &regex::Captures| {
        let first_capture = caps
            .get(1)
            .unwrap()
            .as_str()
            .to_lowercase()
            .replace(' ', "-");
        format!("[{}]{{:{}:}}", caps.get(2).unwrap().as_str(), first_capture)
    });
    transformed_text.to_string()
}

fn transform_simple_obsidian_links(text: &str) -> String {
    let re = Regex::new(r"\[\[([^|\]]+)\]\]").unwrap();
    let transformed_text = re.replace_all(text, |caps: &regex::Captures| {
        let first_capture = caps
            .get(1)
            .unwrap()
            .as_str()
            .to_lowercase()
            .replace(' ', "-");
        format!("{{:{}:}}", first_capture)
    });
    transformed_text.to_string()
}

fn transform_markdown_links(text: &str) -> String {
    let re = Regex::new(r"\[(.*?)\]\((.*?)\)").unwrap();
    let replaced_text = re.replace_all(text, "[$1]{$2}");
    replaced_text.into_owned()
}

fn transform_obsidian(text: &str) -> String {
    let mut text = transform_codeblocks(text);
    text = transform_simple_obsidian_links(&text);
    text = transform_obsidian_links(&text);
    text = transform_markdown_links(&text);
    text = transform_italic(&text); // this has to go before bold
    text = transform_bold(&text);
    text = transform_markdown_lists(&text);
    text = transform_headings(&text);
    text
}

fn main() {
    let matches = clap::Command::new("obsidian2neorg")
        .bin_name("obsidian2neorg")
        .version("0.1.0")
        .author("jonboh")
        .about("Transform obsidian markdown into neorg")
        .after_help(
            r#"Takes input in stdin and outputs the transformation in stdout.
To transform a file do:
cat file.md | obsidian2neorg > file.norg
                    "#,
        )
        .get_matches();
    let mut input_text = String::new();
    io::stdin()
        .read_to_string(&mut input_text)
        .expect("Failed to read input");
    let neorg_text = transform_obsidian(&input_text);
    io::stdout()
        .write_all(neorg_text.as_bytes())
        .expect("Failed to write output");
}

#[cfg(test)]
mod tests {

    use crate::{
        transform_bold, transform_codeblocks, transform_double_underscore_bold, transform_headings,
        transform_italic, transform_markdown_links, transform_markdown_lists, transform_obsidian,
        transform_obsidian_links, transform_simple_obsidian_links,
    };

    #[test]
    fn simple_links() {
        assert_eq!(
            &transform_simple_obsidian_links(
                r#"
This is a [[some content]] and [[another link | with title]]."#
            ),
            r#"
This is a {:some-content:} and [[another link | with title]]."#
        );
        assert_eq!(
            &transform_simple_obsidian_links(
                r#"
This is a [[some content]] and [[another link|with title]]."#
            ),
            r#"
This is a {:some-content:} and [[another link|with title]]."#
        );
    }

    #[test]
    fn lists() {
        assert_eq!(
            &transform_markdown_lists(
                r#"- this
- list
    - should be transfomed"#
            ),
            r#"- this
- list
-- should be transfomed"#
        );
    }

    #[test]
    fn obsidian_links() {
        assert_eq!(
            &transform_obsidian_links(
                r#"
This is a [[some content]] and [[another link|with an alias]]. And a [markdown](http://link.com)"#
            ),
            r#"
This is a [[some content]] and [with an alias]{:another-link:}. And a [markdown](http://link.com)"#
        );
    }

    #[test]
    fn markdown_links() {
        assert_eq!(
            &transform_markdown_links(
                r#"
This is a [[some content]] and [[another link|with an alias]]. And a [markdown](http://link.com)"#
            ),
            r#"
This is a [[some content]] and [[another link|with an alias]]. And a [markdown]{http://link.com}"#
        );
    }

    #[test]
    fn italic() {
        assert_eq!(
            &transform_italic(
                r#"
This is a **some bold text** and *some italic text*."#
            ),
            r#"
This is a **some bold text** and /some italic text/."#
        );
    }

    #[test]
    fn codeblock() {
        assert_eq!(
            &transform_codeblocks(
                r#"
Look some code:

```rust
let x = 5

todo!()
```"#
            ),
            r#"
Look some code:

@code rust
let x = 5

todo!()
@end"#
        );
    }

    #[test]
    fn bold() {
        assert_eq!(
            &transform_bold(
                r#"
this is some **bold** text"#
            ),
            r#"
this is some *bold* text"#
        );
        assert_eq!(
            &transform_double_underscore_bold(
                r#"
this is some __bold__ text"#
            ),
            r#"
this is some *bold* text"#
        );
        assert_eq!(
            &transform_bold(
                r#"
this is also**bold**text"#
            ),
            r#"
this is also*bold*text"#
        );
    }
    #[test]
    fn headings() {
        assert_eq!(
            &transform_headings(
                r#"
# This is a heading"#
            ),
            r#"
* This is a heading"#
        );

        assert_eq!(
            &transform_headings(
                r#"
## This is a second level heading"#
            ),
            r#"
** This is a second level heading"#
        );

        assert_eq!(
            &transform_headings(
                r#"
# This is a first level
[[maybe]] [[some link/tags]]
## This is a second level"#
            ),
            r#"
* This is a first level
[[maybe]] [[some link/tags]]
** This is a second level"#
        );
        assert_eq!(
            &transform_headings(
                r#"
### This is a third level
## This is a second level

Text and content"#
            ),
            r#"
*** This is a third level
** This is a second level

Text and content"#
        );
        assert_eq!(
            &transform_headings(
                r#"
    ### This is not a third level
## This is a second level
This is some text"#
            ),
            r#"
    ### This is not a third level
** This is a second level
This is some text"#
        );
        assert_eq!(
            &transform_headings(
                r#"
## This is a second level and next is not # a heading
And here is some text next to a heading"#
            ),
            r#"
** This is a second level and next is not # a heading
And here is some text next to a heading"#
        );
    }

    #[test]
    fn all_substitutions() {
        assert_eq!(
            &transform_obsidian(
                r#"
## This is a l2 heading
This is a [[some link]] and [[another link|with title]].
There is also a [link to github](https://github.com/jonboh)
Then there's some text, **bold text** and some *italic text* as well
- We have
- numbered items as well
    - and sublists
        - as well
and code!
```rust
match res {
    Ok(result) => { }
    Err(_) => { }
}

enum Some {
    Variant,
    OtherVariant,
}
```
"#
            ),
            r#"
** This is a l2 heading
This is a {:some-link:} and [with title]{:another-link:}.
There is also a [link to github]{https://github.com/jonboh}
Then there's some text, *bold text* and some /italic text/ as well
- We have
- numbered items as well
-- and sublists
--- as well
and code!
@code rust
match res {
    Ok(result) => { }
    Err(_) => { }
}

enum Some {
    Variant,
    OtherVariant,
}
@end
"#
        );
    }
}
