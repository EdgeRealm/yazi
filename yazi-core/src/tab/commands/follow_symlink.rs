use yazi_config::keymap::Exec;
use crate::tab::Tab;


pub struct Opt { }


impl<'a> From<&'a Exec> for Opt {
	fn from(_: &'a Exec) -> Self {
		Self {  }
	}
}


impl Tab {
    pub fn follow_symlink(&mut self, _: impl Into<Opt>) -> bool {
        let url = self.current.hovered().unwrap().link_to().unwrap().clone();
		self.cd(url);
        false
    }

}
