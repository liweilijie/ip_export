use clap::{Arg, App};
use std::thread;
use std::time::Duration;
mod dnspod;
mod mail;
mod comm;

#[tokio::main]
async fn main() {
    let matches = App::new("export ip address")
        .version("0.0.1")
        .author("liwei <liweilijie@gmail.com>")
        .about("do export ip address")
        .arg(Arg::with_name("record")
            .short('r')
            .long("record")
            .value_name("FILE")
            .about("to write ip address while ip changed.")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("period")
            .short('t')
            .long("period")
            .value_name("PERIOD")
            .about("the period of lookup ip")
            .takes_value(true)
        )
        .get_matches();

    let record = matches.value_of("record").unwrap_or("record");
    let period: u64 = matches.value_of_t("period").unwrap_or(3600u64);
    println!("record:{}, period:{}", record, period);
    let period = Duration::from_secs(period);
    let failed_period: Duration = Duration::from_secs(120);

    // 读取文件中保存的ip地址
    let mut old_ip = match comm::read_contents(record) {
        Ok(ip) => ip,
        Err(_) => String::new(),
    };

    if old_ip.is_empty() {
        println!("old ip is empty.");
    } else {
        println!("old ip is: {}", old_ip);
    }

    // user-agent pools
    let ua: [&str; 4] = [
        "Mozilla/5.0 (Macintosh; U; Intel Mac OS X 10_6_8; en-us) AppleWebKit/534.50 (KHTML, like Gecko) Version/5.1 Safari/534.50",
        "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/39.0.2171.71 Safari/537.36",
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.11 (KHTML, like Gecko) Chrome/23.0.1271.64 Safari/537.11",
        "Mozilla/5.0 (Windows; U; Windows NT 6.1; en-US) AppleWebKit/534.16 (KHTML, like Gecko) Chrome/10.0.648.133 Safari/534.16"];

    // 获取ip地址
    let ip_pools: [&str; 5] = [
        "http://httpbin.org/ip",
        "http://ip.42.pl/raw",
        "http://jsonip.com",
        "https://api.ipify.org/?format=json",
        "http://www.cip.cc"];

    loop {
        let mut epoch: bool = false;
        for url in ip_pools.iter() {
             // println!("url:{}", url);
            let mut ip: String = String::new();
            for a in ua.iter() {
                 // println!("    url:{}, ua:{}", url, a);
                // if let Some(ip) = comm::blocking_get_ip(url) {
                let res = comm::async_get_ip(a, url).await;
                match res {
                    Ok(r) => {
                        if r != "" {
                            ip = r;
                            break;
                        } else {
                            continue;
                        }
                    },
                    Err(e) => {
                        println!("async get ip failed: {}", e);
                        continue;
                    }
                }
            }

            if ip.is_empty() {
                println!("{}, ip is empty so continue.", url);
                continue;
            }

            // println!("success get ip: {}", ip);
            if let Some(number) = comm::parse_ip(&ip) {
                epoch = true;
                // println!("number:{}", number);
                if (!number.is_empty()) && (number != old_ip) {
                    println!("not equal {} <> {}", number, old_ip);

                    // update dnspod
                    let res = dnspod::get_record_info(&number).await;
                    match res {
                        Ok(r) => {
                            if r != "" {
                                println!("update dnspod success: {}", r);
                            } else {
                                println!("update dnspod failed");
                            }
                        },
                        Err(e) => {
                            println!("update dnspod failed: {}", e);
                        }
                    }

                    if mail::send_email(&number) {
                        println!("send email success");
                        match comm::write_contents(record, &number) {
                            Err(e) => println!("write err:{}", e),
                            Ok(_) => println!("write success."),
                        }
                        old_ip = number;
                        break;
                    }
                } else if !number.is_empty() {
                    println!("{} == {} so nothing to do.", number, old_ip);
                    break;
                }
            }
            // } else {
            //     println!("failed get url({}) so continue.", url);
            // }
        }
        if epoch {
            thread::sleep(period);
        } else {
            // 如果所有的url都没有获取到想要的ip值，则睡2分钟继续做。
            thread::sleep(failed_period);
        }
    }
}

