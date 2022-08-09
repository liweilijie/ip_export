// #[tokio::main]
// async fn main() {
//     let home_ip: &str = "218.6.213.130";
//     let res = get_record_info(home_ip).await;
//     match res {
//         Ok(r) => println!("{:#?}", r),
//         Err(e) => println!("{}", e),
//     }
// }

#[derive(Debug, serde::Deserialize)]
struct RecordInfo {
    id: String,
    line_id: String,
    name: String,
    #[serde(rename="type")]
    type_name: String,
    value: String,
}

#[derive(Debug, serde::Deserialize)]
struct RecordResponse {
    records: [RecordInfo;1],
}

#[derive(Debug, serde::Deserialize)]
struct Status {
    code: String,
    message: String,
}
#[derive(Debug, serde::Deserialize)]
struct StatusResponse {
    status: Status,
}

pub async fn get_record_info(home_ip: &str) -> Result<String, reqwest::Error> {
    // 获取id value, line_id等参数
    let r: RecordResponse = reqwest::Client::new()
        .post("https://dnsapi.cn/Record.List")
        .form(&[
            ("login_token", "158816,6355830edd9f9471a9ddb149155b7833"),
            ("format","json"),
            ("domain", "emacsvi.com"),
            ("sub_domain", "home"),
            ("record_type", "A"),
            ("offset", "0"),
            ("length", "1")
        ])
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", r.records[0]);
    println!("id:{}", r.records[0].id.as_str());

    // 将获取到的ip与home_ip对比，如果不相同，则更新
    if home_ip != r.records[0].value {
        println!("should to update ip: {}.", home_ip);
    } else {
        println!("{} == {} nothing to do.", home_ip, r.records[0].value);
        return Ok(String::from(""));
    }

    // update ip
    let u: StatusResponse = reqwest::Client::new()
        // .timeout(Duration::from_secs(10))
        .post("https://dnsapi.cn/Record.Modify")
        // login_token=158816,6355830edd9f9471a9ddb149155b7833
        // &format=json
        // &domain_id=27919306
        // &record_id=584498005
        // &sub_domain=home
        // &value=218.6.240.220
        // &record_type=A
        // &record_line_id=10%3D0
        .form(&[
            ("login_token", "158816,6355830edd9f9471a9ddb149155b7833"),
            ("format","json"),
            ("domain", "emacsvi.com"),
            ("record_id", r.records[0].id.as_str()),
            ("sub_domain", "home"),
            ("value", home_ip),
            ("record_type", "A"),
            ("record_line_id", r.records[0].line_id.as_str())
        ])
        .header("User-Agent", "liwei ddns/1.0.0(lijieliwei@126.com)")
        .send()
        .await?
        .json()
        .await?;

    println!("status:{:#?}", u.status);
    if u.status.code == "1" {
        Ok(String::from(home_ip))
    } else {
        Ok(String::from(""))
    }
}
