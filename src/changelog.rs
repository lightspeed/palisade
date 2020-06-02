use anyhow::Result;
use comrak::nodes::{AstNode, NodeValue};
use comrak::{format_commonmark, parse_document, Arena, ComrakOptions};
use std::fs::read_to_string;
use std::path::PathBuf;

pub(crate) fn read(fname: PathBuf, tag: String) -> Result<String> {
    let data = read_to_string(fname)?;
    let arena = Arena::new();
    let mut root = parse_document(&arena, &data, &ComrakOptions::default());

    let mut collect = false;
    let mut buf = Vec::<u8>::new();

    iter_nodes(&mut root, &mut |node| {
        let nd = node.data.borrow();

        match nd.value {
            NodeValue::Heading(ref hdr) => {
                if hdr.level == 2 {
                    if collect {
                        collect = false;
                    }

                    let found_tag = String::from_utf8(nd.content.clone())?;

                    if found_tag == tag {
                        collect = true;
                    }
                } else {
                    if collect {
                        format_commonmark(&node, &ComrakOptions::default(), &mut buf)?;
                    }
                }
                Ok(())
            }
            NodeValue::Item(_) => Ok(()),
            _ => {
                if collect {
                    format_commonmark(&node, &ComrakOptions::default(), &mut buf)?;
                }

                Ok(())
            }
        }
    })?;

    Ok(String::from_utf8(buf)?)
}

fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &mut F) -> Result<()>
where
    F: FnMut(&'a AstNode<'a>) -> Result<()>,
{
    f(node)?;
    for c in node.children() {
        match c.data.borrow().value {
            NodeValue::Text(_) => {}
            NodeValue::Item(_) => {}
            NodeValue::Code(_) => {}
            _ => {
                iter_nodes(c, f)?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn read_changelog() {
        let res = super::read("testdata/basic.md".into(), "0.1.0".into());
        assert!(res.is_ok());
        let delta = res.unwrap();
        assert_eq!(delta, "Hi there this is a test\\!\n### ADDED\n  - something\n")
    }
}
