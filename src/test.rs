use std::fs;
use std::io::prelude::*;
use std::io::Write;

pub fn test_code() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout_color: i8;
    let test_dir = "test";
    let test_files = fs::read_dir(test_dir).unwrap();
    let mut problem_num = 0;
    let mut ac_num = 0;
    for test_file in test_files {
        let path_name = format!("{}", test_file.unwrap().path().display());
        let split_path_name: Vec<&str> = path_name.split('.').collect();
        let file_name_without_extension = split_path_name[0];
        if split_path_name[1] == "out" {
            continue;
        }
        problem_num += 1;
        stdout_color = 4;
        print!("[\x1b[{}mINFO\x1b[m] ", 30 + stdout_color);
        println!("case - {}:", file_name_without_extension);
        // テストディレクトリから標準入力用ファイルを取得する
        let input_string = fs::read(format!("{}.in", file_name_without_extension))
            .unwrap()
            .iter()
            .map(|&s| s as char)
            .collect::<String>();
        let process = match std::process::Command::new("./a.out")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
        {
            Err(why) => panic!("couldn't spawn: {}", why),
            Ok(process) => process,
        };
        match process.stdin.unwrap().write_all(input_string.as_bytes()) {
            Err(why) => panic!("couldn't write to ./a.out stdin: {}", why),
            Ok(_) => {}
        }
        let mut user_ans = String::new();
        match process.stdout.unwrap().read_to_string(&mut user_ans) {
            Err(why) => panic!("couldn't read ./a.out stdout: {}", why),
            Ok(_) => {}
        }
        let correct_ans = fs::read(format!("{}.out", file_name_without_extension))
            .unwrap()
            .iter()
            .map(|&s| s as char)
            .collect::<String>();
        if user_ans == correct_ans {
            stdout_color = 2;
            ac_num += 1;
            print!("[\x1b[{}mSUCCESS\x1b[m] ", 30 + stdout_color);
            println!("\x1b[{}mAC\x1b[m\n", 30 + stdout_color);
        } else {
            stdout_color = 1;
            print!("[\x1b[{}mFAILURE\x1b[m] ", 30 + stdout_color);
            println!("\x1b[{}mWA\x1b[m", 30 + stdout_color);
            println!("input:\n{}", input_string);
            println!("output:\n{}", user_ans);
            println!("expected:\n{}", correct_ans);
        }
    }
    if ac_num == problem_num {
        stdout_color = 2;
        print!("[\x1b[{}mSUCCESS\x1b[m] ", 30 + stdout_color);
        println!("{} AC / {} cases", ac_num, problem_num);
    } else {
        stdout_color = 1;
        print!("[\x1b[{}mFAILURE\x1b[m] ", 30 + stdout_color);
        println!("{} AC / {} cases", ac_num, problem_num);
    }
    return Ok(());
}
