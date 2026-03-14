use std::path::PathBuf;
use std::process;

use clap::{Parser, Subcommand};
use docusaurus::{run_command, RunnerOptions};

#[derive(Parser)]
#[command(name = "docusaurus", about = "Docusaurus CLI with Rust-native config")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Build {
        #[arg(default_value = ".")]
        site_dir: PathBuf,
        #[arg(long)]
        out_dir: Option<String>,
        #[arg(long, num_args = 1..)]
        locale: Vec<String>,
        #[arg(long)]
        dev: bool,
        #[arg(long)]
        bundle_analyzer: bool,
        #[arg(long = "no-minify")]
        no_minify: bool,
    },
    Start {
        #[arg(default_value = ".")]
        site_dir: PathBuf,
        #[arg(long)]
        port: Option<u16>,
        #[arg(long)]
        host: Option<String>,
        #[arg(long)]
        hot_only: bool,
        #[arg(long = "no-open")]
        no_open: bool,
    },
    Serve {
        #[arg(default_value = ".")]
        site_dir: PathBuf,
        #[arg(long)]
        port: Option<u16>,
        #[arg(long)]
        host: Option<String>,
        #[arg(long)]
        dir: Option<String>,
        #[arg(long)]
        build: bool,
        #[arg(long = "no-open")]
        no_open: bool,
    },
    Deploy {
        #[arg(default_value = ".")]
        site_dir: PathBuf,
    },
    Clear {
        #[arg(default_value = ".")]
        site_dir: PathBuf,
    },
    Swizzle {
        #[arg(default_value = ".")]
        site_dir: PathBuf,
        theme: Option<String>,
        component: Option<String>,
        #[arg(long)]
        eject: bool,
        #[arg(long)]
        wrap: bool,
        #[arg(long)]
        danger: bool,
        #[arg(long)]
        list: bool,
        #[arg(long)]
        typescript: bool,
    },
    WriteTranslations {
        #[arg(default_value = ".")]
        site_dir: PathBuf,
        #[arg(long)]
        locale: Option<String>,
    },
    WriteHeadingIds {
        #[arg(default_value = ".")]
        site_dir: PathBuf,
        content_paths: Vec<PathBuf>,
        #[arg(long)]
        maintain_case: bool,
        #[arg(long)]
        overwrite: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = dispatch(cli);
    if let Err(e) = result {
        eprintln!("error: {e}");
        process::exit(1);
    }
}

fn dispatch(cli: Cli) -> Result<(), docusaurus::DocusaurusError> {
    match cli.command {
        Commands::Build {
            site_dir,
            out_dir,
            locale,
            dev,
            bundle_analyzer,
            no_minify,
        } => {
            let cli_options = serde_json::json!({
                "outDir": out_dir,
                "locale": if locale.is_empty() { None } else { Some(locale) },
                "dev": dev,
                "bundleAnalyzer": bundle_analyzer,
                "minify": !no_minify,
            });
            run_command("build", RunnerOptions { site_dir, cli_options })
        }
        Commands::Start {
            site_dir,
            port,
            host,
            hot_only,
            no_open,
        } => {
            let cli_options = serde_json::json!({
                "port": port,
                "host": host,
                "hotOnly": hot_only,
                "open": !no_open,
            });
            run_command("start", RunnerOptions { site_dir, cli_options })
        }
        Commands::Serve {
            site_dir,
            port,
            host,
            dir,
            build,
            no_open,
        } => {
            let cli_options = serde_json::json!({
                "port": port,
                "host": host,
                "dir": dir,
                "build": build,
                "open": !no_open,
            });
            run_command("serve", RunnerOptions { site_dir, cli_options })
        }
        Commands::Deploy { site_dir } => {
            run_command("deploy", RunnerOptions { site_dir, cli_options: serde_json::json!({}) })
        }
        Commands::Clear { site_dir } => {
            run_command("clear", RunnerOptions { site_dir, cli_options: serde_json::json!({}) })
        }
        Commands::Swizzle {
            site_dir,
            theme,
            component,
            eject,
            wrap,
            danger,
            list,
            typescript,
        } => {
            let cli_options = serde_json::json!({
                "theme": theme,
                "component": component,
                "eject": eject,
                "wrap": wrap,
                "danger": danger,
                "list": list,
                "typescript": typescript,
            });
            run_command("swizzle", RunnerOptions { site_dir, cli_options })
        }
        Commands::WriteTranslations { site_dir, locale } => {
            let cli_options = serde_json::json!({ "locale": locale });
            run_command("writeTranslations", RunnerOptions { site_dir, cli_options })
        }
        Commands::WriteHeadingIds {
            site_dir,
            content_paths,
            maintain_case,
            overwrite,
        } => {
            let paths: Vec<String> = content_paths
                .iter()
                .map(|p| p.display().to_string())
                .collect();
            let cli_options = serde_json::json!({
                "contentPaths": paths,
                "maintainCase": maintain_case,
                "overwrite": overwrite,
            });
            run_command("writeHeadingIds", RunnerOptions { site_dir, cli_options })
        }
    }
}
