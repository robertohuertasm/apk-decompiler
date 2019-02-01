use console::style;
use std::env;
use std::fs;
use std::io::{copy, Error, ErrorKind, Result};
use std::path::PathBuf;
use std::process::Command;

pub struct Decompiler {
    apk_path: PathBuf,
    current_dir: PathBuf,
    exe_dir: PathBuf,
    output_path: PathBuf,
}

impl Decompiler {
    pub fn new(apk_path: PathBuf) -> Self {
        let current_dir = env::current_dir().unwrap();
        let mut exe_dir = env::current_exe().unwrap();
        exe_dir.pop();
        let output_path = current_dir.join("output");
        Self {
            apk_path,
            current_dir,
            exe_dir,
            output_path,
        }
    }

    pub fn start(&self) -> Result<()> {
        self.check_path()?;
        println!(
            "{}",
            style(format!(
                "Decompiling {} ...",
                self.apk_path.as_path().display()
            ))
            .bold()
            .green()
        );
        self.create_output()?;
        self.unzip()?;
        self.create_jar()?;
        self.decompile_jar()?;
        self.extract_xml()?;
        println!(
            "{}",
            style("Hurray! Your apk has been decompiled! Check out the output folder.")
                .bold()
                .green()
        );
        Ok(())
    }

    fn check_path(&self) -> Result<()> {
        if self.apk_path.exists()
            && self.apk_path.extension().is_some()
            && self.apk_path.extension().unwrap().eq("apk")
        {
            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::NotFound,
                "The path to the apk is not correct",
            ))
        }
    }

    fn create_output(&self) -> Result<()> {
        self.msg("  Managing output folder...");
        if self.output_path.exists() {
            fs::remove_dir_all(&self.output_path)?;
        }
        fs::create_dir(&self.output_path)?;
        self.done()
    }

    fn unzip(&self) -> Result<()> {
        self.msg("  Extracting apk content...");
        let zip_path = self.apk_path.with_extension("zip");
        fs::copy(&self.apk_path, &zip_path)?;

        let reader = fs::File::open(&zip_path)?;
        let mut archive = zip::ZipArchive::new(reader)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = self.output_path.join(format!(
                "extracted/{}",
                file.sanitized_name().as_path().display()
            ));

            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }

            if (file.name()).ends_with('/') {
                let msg = format!(
                    "File {} extracted to \"{}\"",
                    i,
                    outpath.as_path().display()
                );
                self.extract_msg(&msg);
                fs::create_dir_all(&outpath)?;
            } else {
                let msg = format!(
                    "File {} extracted to \"{}\" ({} bytes)",
                    i,
                    outpath.as_path().display(),
                    file.size()
                );
                self.extract_msg(&msg);
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(&p)?;
                    }
                }
                let mut outfile = fs::File::create(&outpath)?;
                copy(&mut file, &mut outfile)?;
            }
        }

        fs::remove_file(&zip_path)?;
        self.done()
    }

    fn create_jar(&self) -> Result<()> {
        self.msg("  Generating a jar file...");
        Command::new("sh")
            .arg(self.exe_dir.join("lib/dex2jar/d2j-dex2jar.sh"))
            .arg("-f")
            .arg(&self.apk_path)
            .arg("-o")
            .arg(self.output_path.join("app.jar"))
            .output()?;

        let rd = fs::read_dir(&self.current_dir)?;

        let error_file = rd
            .map(|f| f.unwrap().file_name().to_str().unwrap().to_owned())
            .find(|s| s.ends_with("-error.zip"));

        if let Some(ref e) = error_file {
            println!(
                "{}",
                style(format!(
                    "    \u{26a0} There were some errors in the decompilation process. Please take a look at: {}",
                    self.output_path.join(e).as_path().display()
                ))
                .bold()
                .yellow()
            );

            self.move_error_zip(e)?;
        }
        self.done()
    }

    fn move_error_zip(&self, error_file: &str) -> Result<()> {
        fs::copy(error_file, self.output_path.join(error_file))?;
        fs::remove_file(error_file)?;
        Ok(())
    }

    fn decompile_jar(&self) -> Result<()> {
        self.msg("  Decompiling jar file...");
        let jar_file = self.output_path.join("app.jar");
        Command::new(self.exe_dir.join("lib/jd/jd-cli"))
            .arg("-od")
            .arg(self.output_path.join("decompiled"))
            .arg(&jar_file)
            .output()?;
        fs::remove_file(jar_file)?;
        self.done()
    }

    fn extract_xml(&self) -> Result<()> {
        self.msg("  Managing output folder...");
        Command::new(self.exe_dir.join("lib/apktool/apktool"))
            .arg("d")
            .arg(&self.apk_path)
            .arg("-o")
            .arg(self.output_path.join("xml"))
            .output()?;
        self.done()
    }

    fn msg(&self, msg: &str) {
        println!("{}", style(msg).cyan());
    }

    fn extract_msg(&self, msg: &str) {
        println!("  .... {}", style(msg).magenta());
    }

    fn done(&self) -> Result<()> {
        println!("{}", style("  ...Done \u{2705}").green());
        Ok(())
    }
}
