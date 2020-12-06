use encoding_rs;

fn download_holiday_csv(url: &str) {
    let resp = reqwest::blocking::get(url).expect("failed to download");
    let body = resp.bytes().expect("failed to get body");
    let (content, _, _) = encoding_rs::SHIFT_JIS.decode(&body[..]);
    let content = content.into_owned();

    // skip CSV header
    for line in content.lines().skip(1) {
        println!("{}", line);
    }
}

fn main() {
    let csv_url = "https://www8.cao.go.jp/chosei/shukujitsu/syukujitsu.csv";
    download_holiday_csv(csv_url);
}
