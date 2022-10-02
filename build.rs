extern crate lalrpop;
extern crate winres;

fn main() {
/*    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("leblanc.ico");
        res.compile().unwrap();
    }*/
    lalrpop::Configuration::new().always_use_colors().process_current_dir().unwrap();
}
