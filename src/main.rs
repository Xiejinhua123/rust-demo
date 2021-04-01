extern crate http;
extern crate systemstat;
extern crate futures;
extern crate hyper;

use std::{fs::{self, OpenOptions}, io::Write, num::ParseIntError, ops::{Index, Sub}, thread, time};

use systemstat::{Duration, Platform, System};

fn main(){
}

async fn ball() -> Result<(), reqwest::Error> {
    let need_char = ["a", "b", "c", "d", "e", "i",
     "l", "o", "r", "s", "u", "v", "x", "<", ">", "_", "/",
     "0","1","2","3","4","5","6","7","8","9"];
     let need_num = ["0","1","2","3","4","5","6","7","8","9"];
    let mut file = OpenOptions::new().append(true).open("D:\\data.txt").expect(
        "cannot open file");
    let text = fs::read_to_string("D:\\a.txt").unwrap();
    let split = text.split("</a>");
    for str in split {
        let url = substr(str, 9, 46);
        let mut out = substr(&url, 35, 5);
        let body = reqwest::get(url).await?.text().await?;
        let mut result = String::new();
        let b = body.split("");
        for char_b in b {
            if need_char.contains(&char_b){
                result += char_b;
            }
        }
        let result_option = result.find("<liclassball_red>");
        if result_option.is_none(){
            continue;
        }
        let index = result_option.unwrap();
        let result = substr(&result, index, 170);
        let result = result.replace("<liclassball_red>", "\t").replace("</li>", "")
        .replace("<liclassball_blue>", "\t").replace("<", "");
        out += &result;
        println!("{:?}", out);
        file.write_all(out.as_bytes()).expect("write failed");
        file.write_all("\n".as_bytes()).expect("write failed");
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