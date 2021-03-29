extern crate http;
extern crate systemstat;
extern crate futures;
extern crate hyper;

use std::{fs, num::ParseIntError, ops::{Index, Sub}, thread, time};

use systemstat::{Duration, Platform, System};

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

    let var_str = "abcdefg";
    let index = var_str.find("c").unwrap();
    let var_str_tmp = substr(&var_str, index, 200);
    println!("{:?}", var_str_tmp);


    let text = fs::read_to_string("D:\\a.txt").unwrap();
    let split = text.split("</a>");
    for str in split {
        let url = substr(str, 9, 46);
        println!("{}", &url);
        print!("{:?}\t", substr(&url, 35, 5));
        let body = reqwest::get(url).await?.text().await?;
        let mut body = body.replace("\t", "").replace("\n", "")
        .replace("\r", "").replace("/", "").replace(" ", "")
        .replace("\"", "");
        let index_option = body.find("ball_box01");
        if index_option.is_none() {
            println!("无");
            continue;
        }else{
            let index = index_option.unwrap();
            print!("{}", index);
            body.truncate(index + 1000);
            println!("{:?}", body);
        }
        break;
    }
    Ok(())
}

// substr 截取字符串
fn substr(s: &str, start: usize, length: usize) -> String {
    s.chars().skip(start).take(length).collect()
}

// system_monitor 系统监控
fn system_monitor() {
    let sys = System::new();

    match sys.cpu_temp() {
        Ok(cpu_temp) => println!("\nCPU temp: {}", cpu_temp),
        Err(x) => println!("\nCPU temp: {}", x)
    }

    match sys.cpu_load_aggregate() {
        Ok(cpu)=> {
            println!("\nMeasuring CPU load...");
            thread::sleep(Duration::from_secs(1));
            let cpu = cpu.done().unwrap();
            println!("CPU load: {}% user, {}% nice, {}% system, {}% intr, {}% idle ",
                cpu.user * 100.0, cpu.nice * 100.0, cpu.system * 100.0, cpu.interrupt * 100.0, cpu.idle * 100.0);
        },
        Err(x) => println!("\nCPU load: error: {}", x)
    }

    println!("myname -- xiejinhua");
}