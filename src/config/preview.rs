use std::fmt::Display;

use crate::{config::OutLvl, ctx_string::Context};

use super::Backup;

const PAGE_WIDTH: usize = 80;
const INDENT: usize = 4;
const SPACE: usize = 2;

impl Backup {
    pub(crate) fn preview(&self, variables: &Context, name: String) {
        let name = format!("BACKUP {}", name);
        let space = " ";
        let taglen = 2 * space.len() + name.len();
        let bar_a = "═".repeat(taglen);
        let bar_b = "═".repeat(PAGE_WIDTH - taglen - 1);
        println!(
            "{bar_a}╦{bar_b}\n\
            {space}{name}{space}║\n\
            {bar_a}╝",
        );

        let source = match self.source.evaluate(&variables) {
            Ok(s) => PointContent::Single(s),
            Err(_) => PointContent::Single(String::from("ERROR")),
        };
        let target = match self.target.evaluate(&variables) {
            Ok(s) => PointContent::Single(s),
            Err(_) => PointContent::Single(String::from("ERROR")),
        };
        let exclude = if self.exclude.is_empty() {
            PointContent::Single(String::from("[]"))
        } else {
            PointContent::Multi(
                self.exclude
                    .iter()
                    .map(|x| match x.evaluate(&variables) {
                        Ok(s) => s,
                        Err(_) => String::from("ERROR"),
                    })
                    .collect::<Vec<_>>(),
            )
        };
        let output = PointContent::Single(
            match self.output {
                OutLvl::Quiet => "quiet",
                OutLvl::Default => "default",
                OutLvl::Verbose => "verbose",
            }
            .to_string(),
        );

        let general = section(
            "GENERAL",
            &[
                ("Source:", source),
                ("Target:", target),
                ("Exclude:", exclude),
                ("Output:", output),
            ],
        );

        println!("{}", general);
    }
}

enum PointContent {
    Single(String),
    Multi(Vec<String>),
}

fn section(title: &str, points: &[(&str, PointContent)]) -> String {
    let max_tag_len = points
        .into_iter()
        .map(|(a, _)| a.to_string().len())
        .reduce(usize::max)
        .unwrap_or_default();

    let points = points
        .into_iter()
        .map(|(t, c)| point(t, c, max_tag_len))
        .collect::<Vec<_>>()
        .join("\n");

    format!("{}\n\n{}", title, points)
}

fn point(tag: impl Display, con: &PointContent, max_tag_len: usize) -> String {
    let tag = tag.to_string();
    let indent = " ".repeat(INDENT);
    let space = " ".repeat(SPACE + max_tag_len - tag.len());

    match con {
        PointContent::Single(con) => format!("{}{}{}{}", indent, tag, space, con),
        PointContent::Multi(s) => s
            .into_iter()
            .enumerate()
            .map(|(i, con)| {
                let tag = if i == 0 { tag.as_str() } else { "" };
                point(tag, &PointContent::Single(con.to_owned()), max_tag_len)
            })
            .collect::<Vec<_>>()
            .join("\n"),
    }
}
