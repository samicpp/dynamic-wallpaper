use std::{env,fs,thread};
use std::time::Duration;
use rand::random;
use std::process::Command;

fn main(){
    let mut args:Vec<String>=env::args().collect();
    let _inv=&args[0];

    if args.len()!=3 {
        println!("{_inv}: Need 2 arguments. Got {}",args.len()-1);
        std::process::exit(1);
    };
    args.remove(0);

    let dir: &str=&args[0];
    let ifall=args[1].clone().parse::<u64>().unwrap();

	//let cmd: &str="pcmanfm --set-wallpaper=";

    let entry=fs::read_dir(dir).unwrap();
    let files: Vec<_>=entry.map(|e|e.unwrap().path()).collect();

    //dbg!(files);
    //dbg!(args);
    loop{
        let tar = (random::<f32>() * files.len() as f32).floor() as usize;

        //println!("{}{}",cmd,files[tar].display());
        //dbg!(&files[tar]);
        //dbg!(&files[tar].display());
        let sel:&str=&files[tar].clone().into_os_string().into_string().unwrap();
        let cmd="--set-wallpaper=".to_owned() + &sel.to_owned();
        let _success=Command::new("pcmanfm").arg(cmd).output().expect("failed");
        println!("{}",sel);

        thread::sleep(Duration::from_millis(ifall));
    };
}
