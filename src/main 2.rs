use rayon::prelude::*;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::sync::{Arc, Mutex};

fn main() {
    let file_path = "input.txt";
    let file = File::open(file_path).unwrap_or_else(|_| {
        eprintln!("无法打开文件或文件不存在");
        std::process::exit(1);
    });

    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    if lines.is_empty() {
        eprintln!("文件为空");
        std::process::exit(1);
    }

    println!(
        "要分割的文件共 {} 行，下面输入 {} 就分割成两个文件。",
        lines.len(),
        lines.len() / 2
    );

    let max_lines: usize = read_max_line_from_user_input(lines.len());

    let groups: Vec<Vec<String>> = lines.chunks(max_lines).map(|group| group.to_vec()).collect();

    let mutex = Arc::new(Mutex::new(()));
    let width = (lines.len() / max_lines).to_string().len();

    groups.par_iter().enumerate().for_each(|(index, group)| {
        let output_file_path = format!("output_{:0width$}.txt", index + 1, width = width);

        let output_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&output_file_path)
            .expect("无法打开或创建输出文件");

        write_to_file(output_file, group, &mutex);
    });

    print!("分割文件成功！按Enter键退出程序！");
    io::stdout().flush().expect("刷新输出缓冲区失败");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("无法读取用户输入");
}

fn read_max_line_from_user_input(line_count: usize) -> usize {
    loop {
        print!("这里输入分割后的文件的最大行数：");
        io::stdout().flush().expect("刷新输出缓冲区失败");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("无法读取用户输入");

        match input.trim().parse::<usize>() {
            Ok(num) if line_count / num < 5000 => return num,
            _ => continue,
        }
    }
}

fn _read_file_content(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("无法打开文件");
    let reader = BufReader::new(file);
    reader.lines().map(|line| line.expect("读取行失败")).collect()
}

fn write_to_file(output_file: File, lines: &[String], mutex: &Mutex<()>) {
    let _lock = mutex.lock().unwrap();
    let mut file = io::BufWriter::new(output_file);

    for line in lines {
        writeln!(file, "{}", line).expect("写入行失败");
    }
}