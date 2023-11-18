use std::process::Command;
use yazi_shared::{Url, expand_path};
use std::path::PathBuf;
use yazi_config::keymap::Exec;

use crate::tab::Tab;


pub struct Opt {
	type_:   OptType,
    state: Option<bool>,
}


#[derive(PartialEq, Eq)]
pub enum OptType {
	None,
	RevealFinder,
	FromFinder,
}

impl<'a> From<&'a Exec> for Opt {
	fn from(e: &'a Exec) -> Self {
		Self {
			type_: match e.args.first().map(|s| s.as_str()) {
				Some("from_finder") => OptType::FromFinder,
                Some("reveal_finder") => OptType::RevealFinder,
				_ => OptType::None,
			},
            state: match e.named.get("state").map(|s| s.as_bytes()) {
				Some(b"true") => Some(true),
				Some(b"false") => Some(false),
				_ => None,
			},
        }
	}
}


impl Tab {
    pub fn edge_finder(&mut self, opt: impl Into<Opt>) -> bool {
		let opt = opt.into() as Opt;
		if opt.type_ == OptType::None {
			return false;
		}

        if opt.type_ == OptType::RevealFinder {
            let child = Command::new("osascript").arg("/Users/edge/EdgeDocuments/EdgeGadgets/Apps/yazi/yazi-plugin/edgeplug/edgeRevealFinder.scpt").output().expect("");
            let selected = String::from_utf8_lossy(&child.stdout).trim().to_string();
            if selected.is_empty() {
                return false
            }

            let selected_list: Vec<String> = selected.split(", ").map(|s| s.to_string()).collect();
            let mut pb = PathBuf::new();
            pb.push(&selected_list[0]); // root
            // let yzfile = Url::from(pb.parent().unwrap());
            self.cd(Url::from(expand_path(pb)));

    
            for file in selected_list.clone() {
                let mut pb = PathBuf::new();
                pb.push(file);
                // let yzfile = Url::from(pb);
                self.current.files.select(&Url::from(expand_path(pb)), opt.state);
            }
            true
        } else {
            let folder = self.current.cwd.to_str().unwrap();
            let cmd_make_window = Command::new("/Users/edge/Applications/anaconda3/envs/daily/bin/python").arg("/Users/edge/EdgeDocuments/EdgeGadgets/Apps/yazi/yazi-plugin/edgeplug/edgeFromFinderWindow.py").arg(folder.to_string()).output().expect("");
        
            let selected = String::from_utf8_lossy(&cmd_make_window.stdout).trim().to_string();
            let selected_list: Vec<String> = selected.split(", ").map(|s| s.to_string()).collect();
        
            let mut pb = PathBuf::new();
            pb.push(&selected_list[0]); // root
            // let yzfile = Url::from(pb);
            self.cd(Url::from(expand_path(pb)));
            // let mut pb = PathBuf::new();
            // pb.push(&selected_list[0]); // root
            // let yzfile = Url::from(pb);
            // futures::executor::block_on(
            //     self.cd(Url::from(expand_path(pb)))
            // );
        
            for file in selected_list.clone() {
                let mut pb = PathBuf::new();
                pb.push(file);
                // let yzfile = Url::from(pb);
                self.current.files.select(&Url::from(expand_path(pb)), opt.state);
            }
        
            true        
        };


        false
    }

}
