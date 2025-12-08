use anyhow::{Ok, Result, anyhow};
use tracing_unwrap::{OptionExt, ResultExt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Wiki {
    wiki_type: String,
    wiki_fields: Vec<WikiField>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WikiField {
    Single((String, String)),
    // Multiple is a vec of key, value
    Multiple((String, Vec<(String, String)>)),
}

fn unify_eol(s: &mut String) {
    *s = s.replace("\r\n", "\n");
}

fn trim_space(s: String) -> String {
    s.trim_matches([' ', '\t']).to_string()
}

fn trim_right_space(s: String) -> String {
    s.trim_end_matches([' ', '\t']).to_string()
}

fn trim_left_space(s: String) -> String {
    s.trim_start_matches([' ', '\t']).to_string()
}

fn process_input(s: String) -> (String, i32) {
    let mut offset = 2;
    let mut s = s;
    unify_eol(&mut s);
    for c in s.chars() {
        match c {
            '\n' => offset += 1,
            ' ' | '\t' => continue,
            _ => return (s.trim().to_string(), offset),
        }
    }
    (s.trim().to_string(), offset)
}

const PREFIX: &str = "{{Infobox";
const SUFFIX: &str = "}}";

fn read_type(s: &String) -> String {
    let index = match s.find('\n') {
        Some(i) => i,
        None => s
            .find('}')
            // SAFETY: This should never happen if the wiki string is correctly formatted
            .expect_or_log("Failed to read wiki type, should be unreachable"),
    };

    trim_space(s[PREFIX.len() + 1..index].to_string())
}

fn read_start_line(s: &String) -> Result<(String, String)> {
    match s.split_once('=') {
        None => Err(anyhow!("Failed to read start line, expected sign equal")),
        Some((left, right)) => Ok((
            trim_right_space(left.to_string()),
            trim_left_space(right.to_string()),
        )),
    }
}

fn read_array_item(s: &String) -> Result<(String, String)> {
    if !s.starts_with('[') || !s.ends_with(']') {
        return Err(anyhow!("Wiki array not closed"));
    }
    let content = &s[1..s.len() - 1].to_string();
    match content.split_once('|') {
        None => Ok((String::new(), trim_space(content.to_string()))),
        Some((left, right)) => Ok((trim_space(left.to_string()), trim_space(right.to_string()))),
    }
}

pub fn parse(wiki_str: String) -> Result<Wiki> {
    let mut wiki = Wiki {
        wiki_type: String::new(),
        wiki_fields: Vec::new(),
    };

    let (s, line_offset) = process_input(wiki_str);
    if s.is_empty() {
        return Ok(wiki);
    }

    if !s.starts_with(PREFIX) {
        return Err(anyhow!("Invalid prefix for wiki string"));
    }

    if !s.ends_with(SUFFIX) {
        return Err(anyhow!("Invalid suffix for wiki string"));
    }

    let eol_count = s.chars().filter(|c| *c == '\n').count();

    wiki.wiki_type = read_type(&s);

    if eol_count <= 1 {
        return Ok(wiki);
    }

    let mut item_container = Vec::<(String, String)>::with_capacity(eol_count - 2);
    let mut in_array = false;
    let mut current_field = WikiField::Single((String::new(), String::new()));
    // SAFETY: This should never happen if the wiki string is correctly formatted
    let first_eol = s
        .find('\n')
        .expect_or_log("Failed to find first eol, should be unreachable");

    let mut second_last_eol = 0;
    let mut last_eol: usize = first_eol + 1;
    let mut lino = line_offset - 1;
    let mut line: String;

    loop {
        match s[last_eol..].find('\n') {
            Some(offset) => {
                line = s[last_eol..last_eol + offset].to_string();
                second_last_eol = last_eol;
                last_eol = last_eol + offset + 1;
                lino += 1;
            }
            None => {
                if in_array {
                    return Err(anyhow!(
                        "Wiki array not closed on line: {}, for string {}",
                        lino + 1,
                        s[second_last_eol..last_eol].to_string()
                    ));
                }
                break;
            }
        }

        line = trim_space(line);

        if line.is_empty() {
            continue;
        }

        if line.starts_with('|') {
            // new field
            if in_array {
                return Err(anyhow!(
                    "Wiki array not closed on line: {}, for string {}",
                    lino,
                    line
                ));
            }

            let (key, value) = read_start_line(&trim_left_space(line[1..].to_string()))?;
            match value.as_str() {
                "" => {
                    wiki.wiki_fields
                        .push(WikiField::Single((key.clone(), value.clone())));
                }
                "{" => {
                    in_array = true;
                    current_field = WikiField::Multiple((key.clone(), Vec::new()));
                }
                _ => {
                    wiki.wiki_fields
                        .push(WikiField::Single((key.clone(), value.clone())));
                }
            }
            continue;
        }

        if in_array {
            if line == "}" {
                in_array = false;
                match &mut current_field {
                    WikiField::Single(_) => unreachable!(),
                    WikiField::Multiple((_, current_value)) => {
                        *current_value = std::mem::take(&mut item_container);
                    }
                }
                wiki.wiki_fields.push(current_field.clone());
                continue;
            }
            let (key, value) = read_array_item(&line)?;
            item_container.push((key, value));
        }

        if !in_array {
            return Err(anyhow!("Expecting new fields"));
        }
    }

    Ok(wiki)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use regex::Regex;

    fn expected_wiki() -> Wiki {
        Wiki {
            wiki_type: "Crt".to_string(),
            wiki_fields: vec![
                WikiField::Single(("简体中文名".to_string(), "水树奈奈".to_string())),
                WikiField::Multiple((
                    "别名".to_string(),
                    vec![
                        ("".to_string(), "第二中文名".to_string()),
                        ("".to_string(), "英文名".to_string()),
                        ("日文名".to_string(), "近藤奈々 (こんどう なな)".to_string()),
                        ("纯假名".to_string(), "みずき なな".to_string()),
                        ("罗马字".to_string(), "Mizuki Nana".to_string()),
                        (
                            "昵称".to_string(),
                            "奈々ちゃん、奈々さん、奈々様、お奈々、ヘッド".to_string(),
                        ),
                        ("其他名义".to_string(), "".to_string()),
                    ],
                )),
            ],
        }
    }

    #[test]
    fn test_parse_full() {
        let wiki_str = r#"{{Infobox Crt
|简体中文名= 水树奈奈
|官网= https://www.mizukinana.jp
|FanClub= https://fanclub.mizukinana.jp
|Twitter= https://twitter.com/NM_NANAPARTY
}}"#;

        let result = parse(wiki_str.to_string()).unwrap();
        let expected = Wiki {
            wiki_type: "Crt".to_string(),
            wiki_fields: vec![
                WikiField::Single(("简体中文名".to_string(), "水树奈奈".to_string())),
                WikiField::Single(("官网".to_string(), "https://www.mizukinana.jp".to_string())),
                WikiField::Single((
                    "FanClub".to_string(),
                    "https://fanclub.mizukinana.jp".to_string(),
                )),
                WikiField::Single((
                    "Twitter".to_string(),
                    "https://twitter.com/NM_NANAPARTY".to_string(),
                )),
            ],
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_full_array() {
        let wiki_str = r#"{{Infobox Crt
|简体中文名= 水树奈奈
|别名={
[第二中文名]
[英文名]
[日文名|近藤奈々 (こんどう なな)]
[纯假名|みずき なな]
[罗马字|Mizuki Nana]
[昵称|奈々ちゃん、奈々さん、奈々様、お奈々、ヘッド]
[其他名义|]
}
}}"#;

        let result = parse(wiki_str.to_string()).unwrap();
        assert_eq!(result, expected_wiki());
    }

    #[test]
    fn test_parse_empty_line() {
        let wiki_str = r#"{{Infobox Crt
|简体中文名= 水树奈奈
|别名={


[第二中文名]
[英文名]
[日文名|近藤奈々 (こんどう なな)]

[纯假名|みずき なな]
[罗马字|Mizuki Nana]
[昵称|奈々ちゃん、奈々さん、奈々様、お奈々、ヘッド]
[其他名义|]

}
}}"#;

        let result = parse(wiki_str.to_string()).unwrap();
        assert_eq!(result, expected_wiki());
    }

    #[test]
    fn test_parse_extra_space() {
        let wiki_str = r#"{{Infobox Crt
|简体中文名= 水树奈奈
| 别名 = {
[第二中文名]
[ 英文名]
[日文名|近藤奈々 (こんどう なな)]
[纯假名 |みずき なな]
[罗马字|Mizuki Nana]
[昵称|奈々ちゃん、奈々さん、奈々様、お奈々、ヘッド]
[其他名义|]

 }
}}"#;

        let result = parse(wiki_str.to_string()).unwrap();
        assert_eq!(result, expected_wiki());
    }

    #[test]
    fn test_array_no_close() {
        let wiki_str = r#"{{Infobox Crt
| 别名 = {

[昵称|奈々ちゃん、奈々さん、奈々様、お奈々、ヘッド]
[其他名义|]
}}"#;

        let err = parse(wiki_str.to_string()).unwrap_err();
        let re = Regex::new("array.*close").unwrap();
        assert!(re.is_match(&format!("{:?}", err)));
    }

    #[test]
    fn test_array_no_close2() {
        let wiki_str = r#"{{Infobox Crt
| 别名 = {

[昵称|奈々ちゃん、奈々さん、奈々様、お奈々、ヘッド]
[其他名义|]
|简体中文名= 水树奈奈
}}"#;

        let err = parse(wiki_str.to_string()).unwrap_err();
        let re_close = Regex::new("array.*closed").unwrap();
        let re_line = Regex::new("line: 6").unwrap();
        let err_str = format!("{:?}", err);
        assert!(re_close.is_match(&err_str));
        assert!(re_line.is_match(&err_str));
    }

    #[test]
    fn test_array_no_close_empty_item() {
        let wiki_str = r#"{{Infobox Crt
| 别名 = {
}}"#;

        let err = parse(wiki_str.to_string()).unwrap_err();
        let re_close = Regex::new("array.*closed").unwrap();
        let re_line = Regex::new("line: 3").unwrap();
        let err_str = format!("{:?}", err);
        assert!(re_close.is_match(&err_str));
        assert!(re_line.is_match(&err_str));
    }

    #[test]
    fn test_scalar_no_sign_equal() {
        let wiki_str = r#"{{Infobox Crt
| 别名 
}}"#;

        let err = parse(wiki_str.to_string()).unwrap_err();
        let sign_rgx = Regex::new("sign equal").unwrap();
        let err_str = format!("{:?}", err);
        assert!(sign_rgx.is_match(&err_str));
    }

    #[test]
    fn test_type_no_line_break() {
        let wiki_str = "{{Infobox Crt}}";
        let w = parse(wiki_str.to_string()).unwrap();
        assert_eq!(w.wiki_type, "Crt");
    }

    #[test]
    fn test_error_missing_prefix() {
        let wiki_str = "\n\nNotPrefix Crt\n}}";
        let err = parse(wiki_str.to_string()).unwrap_err();
        assert!(format!("{:?}", err).contains("Invalid prefix"));
    }

    #[test]
    fn test_error_missing_suffix() {
        let wiki_str = "\n\n{{Infobox Crt\n\n\n";
        let err = parse(wiki_str.to_string()).unwrap_err();
        let err = format!("{:?}", err);
        println!("{}", err);
        assert!(err.contains("Invalid suffix"));
    }
}
