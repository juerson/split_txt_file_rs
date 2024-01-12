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

    let max_lines: usize;
    if ask_user_to_split() {
        let file_parts = read_file_parts_from_user_input(lines.len());
        max_lines = lines.len() / file_parts;
    } else {
        max_lines = read_max_line_from_user_input(lines.len());
    }

    // 计算实际生成的份数
    let quotient = div_ceil(lines.len(), max_lines);
    println!(
        "要分割的文件共{}行，每个文件最大行数为{}，实际生成{}个文件。",
        lines.len(),
        max_lines,
        quotient
    );

    let groups: Vec<Vec<String>> = lines
        .chunks(max_lines)
        .map(|group| group.to_vec())
        .collect();

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

fn ask_user_to_split() -> bool {
    loop {
        print!("是否按照文件份数来分割文件？(y/n): ");
        io::stdout().flush().expect("刷新输出缓冲区失败");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("无法读取用户输入");

        let trimmed_input = input.trim().to_lowercase();
        if trimmed_input == "y" {
            return true;
        } else if trimmed_input == "n" {
            return false;
        } else {
            // 继续循环
        }
    }
}

fn read_file_parts_from_user_input(line_count: usize) -> usize {
    println!();
    loop {
        print!(
            "这里输入文件的分割份数(支持2~{}的数字输入，实际生成的文件份数可能多1份)：",
            div_ceil(line_count, 2)
        );
        io::stdout().flush().expect("刷新输出缓冲区失败");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("无法读取用户输入");

        match input.trim().parse::<usize>() {
            Ok(parts) if div_ceil(line_count, 2) >= parts && parts > 1 => return parts,
            _ => continue,
        }
    }
}

fn read_max_line_from_user_input(line_count: usize) -> usize {
    println!();
    loop {
        print!(
            "这里输入分割后，每个文件的最大行数(推荐小于{})：",
            div_ceil(line_count, 2)
        );
        io::stdout().flush().expect("刷新输出缓冲区失败");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("无法读取用户输入");

        match input.trim().parse::<usize>() {
            Ok(num) if line_count / num <= 5000 => return num, // 限定生成的文件份数不能多于5000份
            _ => continue,
        }
    }
}

fn write_to_file(output_file: File, lines: &[String], mutex: &Mutex<()>) {
    let _lock = mutex.lock().unwrap();
    let mut file = io::BufWriter::new(output_file);
    for line in lines {
        writeln!(file, "{}", line).expect("写入行失败");
    }
}

fn div_ceil(x: usize, y: usize) -> usize {
    (x + y - 1) / y
}
