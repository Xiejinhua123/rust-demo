extern crate http;
extern crate systemstat;
extern crate futures;
extern crate hyper;

use std::{fs::{self, OpenOptions}, io::Write, num::ParseIntError, ops::{Index, Sub}, thread, time};

use systemstat::{Duration, Platform, System};

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut file = OpenOptions::new().append(true).open("D:\\data.txt").expect(
        "cannot open file");
    let text = fs::read_to_string("D:\\a.txt").unwrap();
    let split = text.split("</a>");
    for str in split {
        let url = substr(str, 9, 46);
        let mut out = substr(&url, 35, 5) + "\t";
        let body = reqwest::get(url).await?.text().await?;
        let body = body.replace("\t", "").replace("\n", "")
        .replace("\r", "").replace("/", "").replace(" ", "")
        .replace("\"", "").replace("\\", "").replace("f", "")
        .replace("g", "").replace("h", "").replace("j", "")
        .replace("k", "").replace("m", "").replace("n", "")
        .replace("o", "").replace("p", "").replace("q", "")
        .replace("t", "").replace("v", "").replace("w", "")
        .replace("y", "").replace("z", "").replace("{", "")
        .replace("}", "").replace("!", "").replace("-", "")
        .replace(":", "").replace("=", "").replace(";", "")
        .replace("]", "").replace("[", "").replace("+", "")
        .replace(".", "").replace("'", "");
        let ball = substr(&body, 125000, 1000);
        if ball.len() > 0{
            let ball_split = ball.split("<ul>");
            for ball_li in ball_split {
                let li_index = ball_li.find("<liclassball");
                if li_index.is_none(){
                    continue;
                }
                let li_split = ball_li.split("<liclassball");
                for li_str in li_split {
                    let red_str = li_str.find("_red>");
                    if !red_str.is_none(){
                        let red = li_str.replace("_red>", "").replace("<li>", "");
                        out += &(red + "\t");
                    }

                    let blue_str = li_str.find("_blue>");
                    if !blue_str.is_none(){
                        let blue = li_str.replace("_blue>", "").replace("<li>", "");
                        out += &blue;
                    }
                }
            }
        }
        println!("{:?}", out);
        file.write_all(out.as_bytes()).expect("write failed");
        file.write_all("\n".as_bytes()).expect("write failed");
        thread::sleep(time::Duration::from_millis(1000));
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