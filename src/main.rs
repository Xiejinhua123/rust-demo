extern crate http;
extern crate systemstat;

use std::{collections::HashMap, fs, result, thread};

use http::{Request, Response, StatusCode, response};
use reqwest::Response;
use systemstat::{Duration, Platform, System};

fn main() {
    ball();
    let request = Request::get("https://kaijiang.500.com/shtml/ssq/03001.shtml")
      .body(()).unwrap();
    let (parts, body) = request.into_parts();
    println!("{:?}", parts.extensions);
    Response::into_body(request);
}

// ball 获取数据
fn ball(){
    let text = fs::read_to_string("D:\\a.txt").unwrap();
    let split = text.split("</a>");
    for str in split {
        let url = substr(str, 9, 46);
        println!("{:?}", url);
    }
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