use core::arch;
use std::{fs, io, process::Command};

fn main() {

    std::process::exit(real_main())
}

fn real_main() -> i32 {

    let args: Vec<_> = std::env::args().collect();

    // vérifier qu'il y a bien un argument passé en paramètre
    if args.len() < 2 {
        println!("Aucun fichier indiqué pour être décompressé. Utilisation : {} <nom_fichier.zip>", args[0]);
        return 1;
    }

    // Ici, un fichier zip à bien été inidiqué en paramètre

    let fname = std::path::Path::new(&*args[1]);
    let file = fs::File::open(&fname).unwrap();
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
                println!("Fichier {} commentaire : {}", file.name(), comment);
            }
        }

        // cas du dossier
        if (*file.name()).ends_with('/') {
            println!("Dossier {} extrait vers \"{}\"", file.name(), outpath.display());
            if !outpath.exists() {
                fs::create_dir(&outpath).unwrap();
            }
        }
        // cas du fichier
        else {
            println!("Fichier {} extrait vers \"{}\", ({} octets)", file.name(), outpath.display(), file.size());

            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }

            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }

        
    }

    0
}