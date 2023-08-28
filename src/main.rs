use regex_lite::Regex;
use std::{
    env,
    io::{stdout, Write},
    iter::repeat,
    process::Command,
};

static STYLE: &str = "
<style>
.diff-add {
    background-color: green;
}
.diff-del {
    background-color: red;
    text-decoration: line-through;
}
</style>
";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let inp = stdin()
    //     .lock()
    //     .lines()
    //     .skip(5) // skip the git diff header
    //     .collect::<Result<Vec<_>>>()
    //     .unwrap()
    //     .join("\n");

    let mut args = vec!["diff", "--word-diff-regex=\\w+", "-pU99999"];
    let argv = env::args().skip(1).collect::<Vec<_>>();
    for a in argv.iter() {
        args.push(a.as_str());
    }

    let cmd = Command::new("git").args(args).output()?;
    let inp = String::from_utf8(cmd.stdout)?;
    // skip git diff header
    let inp = inp.lines().skip(5).collect::<Vec<_>>().join("\n");

    let out = replace(&inp);
    stdout().lock().write_all(out.as_bytes())?;
    Ok(())
}

fn replace(inp: &str) -> String {
    let re_a1 = Regex::new(r#"(?:\{\+.*\+\}\n?)*\{\+.*\+\}"#).unwrap();
    let re_a2 = Regex::new(r#"\{\+(.*)\+\}"#).unwrap();

    let re_d1 = Regex::new(r#"(?:\[\-.*\-\]\n?)*\[\-.*\-\]"#).unwrap();
    let re_d2 = Regex::new(r#"\[\-(.*)\-\]"#).unwrap();

    let mut end = 0;
    let mut out = String::new();

    let m1 = re_a1.find_iter(&inp).zip(repeat(true));
    let m2 = re_d1.find_iter(&inp).zip(repeat(false));

    let mut v = m1.collect::<Vec<_>>();
    v.append(&mut m2.collect());
    v.sort_by_key(|(m, _)| m.start());

    for (grp, add) in v {
        let re = match add {
            true => &re_a2,
            false => &re_d2,
        };
        let res = re
            .captures_iter(grp.as_str())
            .map(|c| c[1].to_owned())
            .collect::<Vec<_>>()
            .join("\n");
        let res = format!(
            "<span class=\"diff-{}\">{res}</span>",
            if add { "add" } else { "del" }
        );
        out += &inp[end..grp.start()];
        out += &res;
        end = grp.end();
    }
    out += &inp[end..];
    out += STYLE;
    out
}
