use rayon::prelude::*;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::sync::{Arc, Mutex};

fn main() {
    // 读取文件路径
    let file_path = "input.txt";
    // 打开文件
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("无法打开文件或文件不存在");
            std::process::exit(1);
        }
    };
    // 使用BufReader来缓冲读取文件
    let reader = BufReader::new(file);
    // 计算文件的行数
    let line_count = reader.lines().count();
    // 行数为0时，表示文件为空
    if line_count == 0 {
        eprintln!("文件为空");
        std::process::exit(1);
    }

    println!(
        "要分割的文件共 {} 行，下面输入 {} 就分割成两个文件。",
        line_count,
        line_count / 2
    );
    // 获取用户输入的最大行数
    let max_lines: usize = read_max_line_from_user_input(line_count); // 划分每个文件的最大行数

    // 读取文件并分割成组
    let groups: Vec<Vec<String>> = read_file_content(file_path)
        .chunks(max_lines)
        .map(|group| group.to_vec())
        .collect();

    // 创建一个互斥锁，用于多线程写文件时的同步
    let mutex = Arc::new(Mutex::new(()));

    // 以最大数的位数作为宽度
    let width = (line_count / max_lines).to_string().len();

    // 使用Rayon库进行并行处理
    groups.par_iter().enumerate().for_each(|(index, group)| {
        // 拼接输出文件路径，补全前导零
        let output_file_path = format!("output_{:0width$}.txt", index + 1, width = width);

        // 打开或创建输出文件
        let output_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&output_file_path)
            .expect("无法打开或创建输出文件");

        // 写入文件
        write_to_file(output_file, group, &mutex);
    });

    print!("分割文件成功！按Enter键退出程序！");
    io::stdout().flush().expect("刷新输出缓冲区失败"); // 刷新输出缓冲区
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("无法读取用户输入");
}

fn read_max_line_from_user_input(line_count: usize) -> usize {
    loop {
        print!("这里输入分割后的文件的最大行数：");
        io::stdout().flush().expect("刷新输出缓冲区失败"); // 刷新输出缓冲区
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("无法读取用户输入");

        match input.trim().parse::<usize>() {
            Ok(num) if line_count / num < 5000 => return num, // 这里的5000指最多生成不超过5000个文件
            Ok(_) => {
                continue;
            }
            Err(_) => {
                continue;
            }
        }
    }
}

fn read_file_content(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("无法打开文件");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| line.expect("读取行失败"))
        .collect()
}

fn write_to_file(output_file: File, lines: &[String], mutex: &Mutex<()>) {
    // 在互斥锁的锁定范围内执行文件写入操作
    let _lock = mutex.lock().unwrap();
    let mut file = io::BufWriter::new(output_file);

    for line in lines {
        writeln!(file, "{}", line).expect("写入行失败");
    }
}
