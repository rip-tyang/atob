use std::env;
mod av_bv;

fn main() {
    let args: Vec<String> = env::args().collect();
    let param = &args[1];
    let av = param.parse::<u64>();
    if av.is_err() {
        let res = av_bv::av_bv::btoa(param);
        println!("{}", res)
    } else {
        let res = av_bv::av_bv::atob(&av.unwrap());
        println!("{}", res)
    }
}
