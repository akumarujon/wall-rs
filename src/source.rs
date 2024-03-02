use crate::config::Config;
use crate::error::Error;
use octocrab as Git;
use octocrab::models::repos::Release;
use octocrab::{Octocrab, Page};
use reqwest::Client as Http;
use std::io::Cursor;
use std::sync::Arc;
use std::{fs, io};

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

pub struct Source {
    git: Arc<Octocrab>,
    http: Http,
}

impl Source {
    pub fn new(_config: Option<Config>) -> Self {
        Self {
            git: Git::instance(),
            http: Http::builder().user_agent(USER_AGENT).build().unwrap(),
        }
    }

    pub async fn get_versions(&self) -> Result<Page<Release>, Error> {
        let releases = self
            .git
            .repos("akumarujon", "wall-rs-mirror")
            .releases()
            .list()
            .send()
            .await;

        let releases = match releases {
            Ok(r) => r,
            Err(err) => return Err(Error::ReleaseFetchError(err)),
        };

        Ok(releases)
    }

    pub async fn get_latest_version(&self) -> Result<String, Error> {
        let releases = self
            .git
            .repos("akumarujon", "wall-rs-mirror")
            .releases()
            .list()
            .send()
            .await;

        let releases = match releases {
            Ok(r) => r,
            Err(err) => return Err(Error::ReleaseFetchError(err)),
        };

        let latest = match releases.items.first() {
            Some(v) => v,
            None => return Err(Error::NoVersionFound),
        };

        Ok(latest.tag_name.to_owned())
    }

    pub async fn download_file<T>(&self, url: T, dest: T) -> Result<(), Error>
    where
        T: ToString,
    {
        let response = match self.http.get(url.to_string()).send().await {
            Ok(d) => d,
            Err(_) => return Err(Error::NoInternetConnection),
        };

        let mut file = match std::fs::File::create(dest.to_string().clone()) {
            Ok(f) => f,
            Err(_) => return Err(Error::CantCreateDownloadedFile(dest.to_string())),
        };

        let mut content = Cursor::new(match response.bytes().await {
            Ok(b) => b,
            Err(_) => return Err(Error::CantCreateCursorBytes),
        });

        match std::io::copy(&mut content, &mut file) {
            Ok(_) => {}
            Err(_) => return Err(Error::CantCopyBytes),
        };

        Ok(())
    }

    pub fn extract_file<T>(&self, file: T) -> Result<(), Error>
    where
        T: AsRef<str>,
    {
        let fname = std::path::Path::new(file.as_ref());
        let file = fs::File::open(fname).unwrap();

        let mut archive = zip::ZipArchive::new(file).unwrap();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let outpath = match file.enclosed_name() {
                Some(path) => path.to_owned(),
                None => continue,
            };

            {
                let comment = file.comment();
                if !comment.is_empty() {
                    println!("File {i} comment: {comment}");
                }
            }

            if (*file.name()).ends_with('/') {
                println!("File {} extracted to \"{}\"", i, outpath.display());
                fs::create_dir_all(&outpath).unwrap();
            } else {
                println!(
                    "File {} extracted to \"{}\" ({} bytes)",
                    i,
                    outpath.display(),
                    file.size()
                );
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(p).unwrap();
                    }
                }
                let mut outfile = fs::File::create(&outpath).unwrap();
                io::copy(&mut file, &mut outfile).unwrap();
            }

            // Get and Set permissions
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;

                if let Some(mode) = file.unix_mode() {
                    fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
                }
            }
        }

        Ok(())
    }
}
