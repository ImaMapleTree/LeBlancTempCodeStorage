extern crate lalrpop;
extern crate winres;


fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("leblanc_icon.ico");
    res.compile().unwrap();
    lalrpop::Configuration::new().always_use_colors().process_current_dir().unwrap();
}
