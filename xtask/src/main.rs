use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader},
    path::Path,
    process::Command,
};

use clap::Parser;
use color_eyre::eyre::ContextCompat;
use pandoc::{Pandoc, PandocOption};
use tower_http::services::ServeDir;

#[derive(Parser)]
enum Commands {
    Build {
        #[clap(long)]
        release: bool,
    },
    Serve {
        #[clap(long)]
        release: bool,
        #[clap(short, long, default_value = "8300")]
        port: u16,
    },
}

fn build(
    release: bool,
    workspace: &Path,
    posts: &[&str],
    template: &Path,
) -> color_eyre::Result<()> {
    let target = workspace.join(format!(
        "target/{}/html",
        if release { "release" } else { "debug" }
    ));

    std::fs::create_dir_all(&target)?;

    #[derive(serde::Deserialize, serde::Serialize)]
    struct Article {
        title: String,
        summary: String,
        #[serde(skip_deserializing)]
        path: String,
    }

    let mut articles = Vec::new();

    for &post in posts {
        let path = workspace.join(post);
        let desc = path.join("desc.md");

        let desc_file = BufReader::new(File::open(&desc)?);
        let desc_metadata: String = desc_file
            .lines()
            .skip(1)
            .take_while(|s| !matches!(s.as_deref(), Ok("---")))
            .map(|l| match l {
                Ok(mut l) => {
                    l.push('\n');
                    Ok(l)
                }
                e => e,
            })
            .collect::<Result<_, _>>()?;

        let desc_metadata: Article = serde_yaml::from_str(&desc_metadata)?;

        articles.push(Article {
            path: post.into(),
            ..desc_metadata
        });

        let index = path.join(format!("{post}.html"));

        let mut pandoc = Pandoc::new();
        pandoc
            .add_input(&desc)
            .set_output(pandoc::OutputKind::File(index.clone()))
            .add_option(PandocOption::MathJax(None))
            .add_option(PandocOption::Template(template.to_path_buf()));
        pandoc.execute()?;

        let mut command = Command::new("trunk");
        command
            .args(&["build", "--dist"])
            .args(&[target.join(post), index])
            .arg("--public-url")
            .arg(format!("/{post}"))
            .current_dir(&path);
        if release {
            command.arg("--release");
        }

        let status = command.spawn()?.wait()?;
        if !status.success() {
            color_eyre::eyre::bail!("Trunk failed: {status}")
        }
    }

    let mut index = OpenOptions::new()
        .truncate(true)
        .write(true)
        .create(true)
        .open(target.join("index.html"))?;

    let index_template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse_file(workspace.join("index.liquid"))?;
    index_template.render_to(
        &mut index,
        &liquid::object!({
            "blog_name": "Traxy's Math Board",
            "articles": articles,
        }),
    )?;

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> color_eyre::Result<()> {
    let args = Commands::from_args();

    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace = manifest
        .ancestors()
        .nth(1)
        .context("could not get workspace root")?;

    let template = workspace.join("template.html");

    let workspace_manifest = workspace.join("Cargo.toml");
    let workspace_manifest = std::fs::read_to_string(workspace_manifest)?;
    let workspace_manifest: toml::Value = toml::from_str(&workspace_manifest)?;

    let members = workspace_manifest
        .get("workspace")
        .context("no workspace in root manifest")?
        .get("members")
        .context("no members in workspace")?
        .as_array()
        .context("members is not an array")?;

    let posts: Vec<_> = members
        .iter()
        .filter_map(|m| m.as_str())
        .filter(|m| !["xtask", "plotter"].contains(m))
        .collect();

    match args {
        Commands::Build { release } => build(release, workspace, &posts, &template),
        Commands::Serve { release, port } => {
            build(release, workspace, &posts, &template)?;
            let addr = std::net::SocketAddr::from(([127, 0, 0, 1], port));

            let dir = workspace.join(format!(
                "target/{}/html",
                if release { "release" } else { "debug" }
            ));

            let service = ServeDir::new(dir);

            println!("Starting server on http://127.0.0.1:{port}");
            hyper::Server::bind(&addr)
                .serve(tower::make::Shared::new(service))
                .await?;

            Ok(())
        }
    }
}
