use handlebars::Handlebars;
use std::{collections::HashMap, fs::File, io::Read, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Render a handlebars template file using values from the environment")]
struct Args {
    #[structopt(
        short = "a",
        long = "all",
        help = "Expose the entire environment to the template, not just the .env contents"
    )]
    all_env: bool,

    #[structopt(
        short = "v",
        long = "vars",
        help = "Load the variables from this file instead of .env"
    )]
    vars: Option<PathBuf>,

    #[structopt(help = "The template to render")]
    file: PathBuf,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::from_args();
    run(args, std::io::stdout())
}

fn run(args: Args, out: impl std::io::Write) -> Result<(), anyhow::Error> {
    let vars: HashMap<String, String> = match (args.all_env, args.vars) {
        (true, Some(v)) => {
            dotenv::from_path(v.canonicalize()?)?;
            std::env::vars().collect()
        }
        (true, None) => dotenv::vars().collect(),
        (false, Some(v)) => dotenv::from_path_iter(v.canonicalize()?)?.collect::<Result<_, _>>()?,
        (false, None) => dotenv::from_filename_iter(".env")?.collect::<Result<_, _>>()?,
    };

    let mut h = Handlebars::new();
    h.set_strict_mode(true);

    let mut f = File::open(args.file)?;
    let mut template = String::new();
    f.read_to_string(&mut template)?;
    h.render_template_to_write(&template, &vars, out)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{run, Args};
    use anyhow::Result;
    use assert_matches::assert_matches;
    use std::path::PathBuf;
    use std::str::FromStr;

    fn setcwd() -> std::io::Result<()> {
        let current_dir = std::env::current_dir()?;
        if !current_dir
            .file_name()
            .map(|f| f.to_string_lossy().contains("testfiles"))
            .unwrap_or(false)
        {
            std::env::set_current_dir(current_dir.join("testfiles"))?;
        }
        Ok(())
    }

    #[test]
    fn test_all_env() -> Result<()> {
        setcwd()?;
        std::env::set_var("GLOBAL_VALUE", "global test value");
        let args = Args {
            all_env: true,
            vars: None,
            file: PathBuf::from_str("all_env.txt")?,
        };

        let mut output = Vec::new();
        run(args, &mut output)?;

        assert_eq!(
            String::from_utf8_lossy(&output),
            "The value is test value and the global is global test value.\n"
        );

        Ok(())
    }

    #[test]
    fn test_file_only_env() -> Result<()> {
        setcwd()?;

        std::env::set_var("GLOBAL_VALUE", "global test value");
        let args = Args {
            all_env: false,
            vars: None,
            file: PathBuf::from_str("file_only.txt")?,
        };

        let mut output = Vec::new();
        run(args, &mut output)?;

        assert_eq!(
            String::from_utf8_lossy(&output),
            "The value is test value.\n"
        );

        Ok(())
    }

    #[test]
    fn test_file_only_env_nonexistent() -> Result<()> {
        setcwd()?;

        std::env::set_var("GLOBAL_VALUE", "global test value");
        let args = Args {
            all_env: false,
            vars: None,
            file: PathBuf::from_str("all_env.txt")?,
        };

        let mut output = Vec::new();
        let result = run(args, &mut output);
        assert_matches!(result,
            Err(e) => {
                let msg = e.to_string();
                assert!(msg.contains(r##"Variable "GLOBAL_VALUE" not found"##), "Saw error: {}", msg);
            }
        );

        Ok(())
    }
}
