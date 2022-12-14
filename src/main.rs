use std::{
    io::{Read, Write},
    fs::File,
};


use serde_json::{Result};
use serde::{Deserialize, Serialize};


const PSW: &str = "Failed password for";
const KEY: &str = "no matching key";


#[derive(Debug, Serialize, Deserialize)]
struct InvalidPassword {
    pub date: String,
    pub hostname: String,
    pub time: String,
    pub user_invalid: bool,
    pub username: String,
    pub ip: String,

    
}

#[derive(Debug, Serialize, Deserialize)]
struct InvalidKey {
    pub date: String,
    pub hostname: String,
    pub time: String,
    pub ip: String,

}



fn main() {


    
    let log = fs_to_string("log.txt")
        .expect("Error reading log.txt");



    let log_vec = log.split('\n').collect::<Vec<&str>>();

    let mut allvec: Vec<Vec<String>> = vec![];



    for x in 0..log_vec.len() {

        let log_vec_args_str = log_vec[x].split(' ').collect::<Vec<&str>>();

        let mut log_vec_args = vec![];

        for y in 0..log_vec_args_str.len() {
            log_vec_args.push(format!("{}", log_vec_args_str[y]));
        };

        allvec.push(log_vec_args);
    };




    // this process follows 3 steps 
    // 1: turn readable text into vectors of key words 
    // 2: deserialize those key words into a struct
    // 3: save the data as jsons


    let psw: Vec<InvalidPassword> = invalid_password(allvec.clone());   
    let inval_password_str: String = serde_json::to_string(&psw).unwrap();
    string_to_fs("InvalidPassword.txt", &inval_password_str);

    let key: Vec<InvalidKey> = invalid_key(allvec);
    let inval_key_str: String = serde_json::to_string(&key).unwrap();
    string_to_fs("InvalidKey.txt", &inval_key_str);

            
    // example of interpreting threat data
    interpret();

}

fn interpret() {


    let raw = fs_to_string("InvalidPassword.txt")
        .expect("Failed to read InvalidPassword.txt");

    let psw: Vec<InvalidPassword> = serde_json::from_str(&raw)
        .expect("Failed to deserialize InvalidPassword.txt");

     println!("REPORT\nThere have been {} Failed password attempts between {} and {}",
             psw.len(), psw[0].date, psw[psw.len()-1].date);


    let mut root   = 0; // for line 88
    
    for x in 0..psw.len() {

        let name = &psw[x].username;

        match &name as &str {
            "root" => root += 1,
            // expand this with your systems users!
            _      => {},
        };

    };
    println!("There were {} attempts for root and {} invalid user attempts",
             root,  psw.len() - root );

}
fn invalid_key(raw: Vec<Vec<String>>) -> Vec<InvalidKey> {

    
    let mut inval: Vec<Vec<String>> = vec![];

    // greper functions similarly to grep as it checks given key words against
    // a data set
    for x in 0..raw.len() {
 
        if raw[x].len() > 17 {
            let greper = format!("{} {} {}", raw[x][12], raw[x][13], raw[x][14]);

            if greper == KEY {
                inval.push(raw[x].clone());
            };
        };
    };

    let mut keyvec: Vec<InvalidKey> = vec![];

    // from the entries that have been 'grepered', deserialize the data
    for x in 0..inval.len() {

        let temp = InvalidKey {

            date: format!("{} {}", inval[x][0].clone(), inval[x][1].clone()),
            hostname: inval[x][3].clone(),
            time: inval[x][2].clone(),
            ip: inval[x][9].clone(),

        };

         keyvec.push(temp);

    };

    return keyvec;




}

fn invalid_password(raw: Vec<Vec<String>>) -> Vec<InvalidPassword> {
     
    let mut inval: Vec<Vec<String>> = vec![];

    for x in 0..raw.len() {

        if raw[x].len() > 10 {

 
            let greper = format!("{} {} {}", raw[x][5], raw[x][6], raw[x][7]);
         
            if greper == PSW {
                inval.push(raw[x].clone());
            };
        };
    };

    let mut structured_vec: Vec<InvalidPassword> = vec![];


    for x in 0..inval.len() {


        let (is_user_invalid, username) = match &inval[x][8] as &str{
            "invalid" => (true, inval[x][10].clone()),
            _         => (false, inval[x][8].clone()),
        };
    
        
        let temp = InvalidPassword {
            date: format!("{} {}", inval[x][0].clone(), inval[x][1].clone()),
            time: inval[x][2].clone(),
            user_invalid: is_user_invalid,
            hostname: inval[x][3].clone(),
            username: username,
            ip: inval[x][11].clone(),
        };
        structured_vec.push(temp);
    };

    return structured_vec

}



// filesystem library 

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


