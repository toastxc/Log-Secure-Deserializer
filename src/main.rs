use std::{
    io::{Read, Write},
    fs::File,
};


use serde_json::{Result};
use serde::{Deserialize, Serialize};


const INVAL: &str = "Connection closed by invalid user";
   
#[derive(Debug, Serialize, Deserialize)]
struct InvalidUserData {
    pub date: String,
    pub hostname: String,
    pub time: String,
    pub username: String,
    pub ip: String,
}




fn main() {


    let log_res = fs_to_string("log.txt");


    let log = match log_res {
        Ok(a) => a,
        Err(e) => {println!("invalid: {e}"); return},
    };

    let entries = log.split('\n').collect::<Vec<&str>>();

    let mut allvec: Vec<Vec<String>> = vec![];

    for x in 0..entries.len() {

        let inner = entries[x].split(' ').collect::<Vec<&str>>();

        let mut inner2 = vec![];

        for y in 0..inner.len() {
            inner2.push(format!("{}", inner[y]));
        };

        allvec.push(inner2);
    };


  

    let inval: Vec<InvalidUserData> = invalid_user(allvec);


    let inval_str: String = serde_json::to_string(&inval).unwrap();

    string_to_fs("InvalidUserLogin.txt", &inval_str);


}

fn invalid_user(mut raw: Vec<Vec<String>>) -> Vec<InvalidUserData> {


     
    let mut inval: Vec<Vec<String>> = vec![];

    for x in 0..raw.len() {

        if raw[x].len() > 10 {

 
         
            let greper = format!("{} {} {} {} {}",
                                 raw[x ][5], raw[x ][6], raw[x ][7], raw[x ][8], raw[x ][9]);    
         
            if greper == INVAL {
                inval.push(raw[x].clone());
            };
        };
    };

    let mut structured_vec: Vec<InvalidUserData> = vec![];

    for x in 0..inval.len() {

        let temp = InvalidUserData {
            date: format!("{} {}", inval[x][0].clone(), inval[x][1].clone()),
            time: inval[x][2].clone(),
            hostname: inval[x][3].clone(),
            username: inval[x][10].clone(),
            ip: inval[x][11].clone(),
        };
        structured_vec.push(temp);
    };

    return structured_vec

}



// library

fn string_to_fs(target: &str, payload: &str)  {

    let mut file = File::create(target)
        .expect("could not create file");

      file.write_all(payload.as_bytes()).unwrap();


}

fn fs_to_string(target: &str) -> Result<String> {

 
    let mut file = File::open(target)
        .expect("could not find file");

    let mut out = String::new();
    file.read_to_string(&mut out)
        .expect("could to parse file");

    Ok(out)


}


