use std::fs;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;


/// Запись в бинарный файл
pub fn write_binary_file(filename: &str, data: &[u8]) -> io::Result<()> {
    // Проверка имени файла
    if filename.is_empty() {
        eprintln!("Error: Filename cannot be empty");
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Empty filename",
        ));
    }

    // Проверка пути на валидность
    let path = Path::new(filename);
    if path.is_dir() {
        eprintln!("Error: Path '{}' is a directory", filename);
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Path is directory",
        ));
    }

    if let Some(parent) = path.parent() {
        if !parent.exists() {
            match fs::create_dir_all(parent) {
                Ok(_) => println!("Created parent directories for '{}'", filename),
                Err(e) => {
                    eprintln!("Error creating directories for '{}': {}", filename, e);
                    return Err(e);
                }
            }
        }
    }

    // Проверка данных
    if data.is_empty() {
        eprintln!("Warning: Writing empty data to file '{}'", filename);
    }

    // Создание файла с обработкой ошибок
    let mut output = match File::create(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error creating file '{}': {}", filename, e);
            return Err(e);
        }
    };

    // Запись данных с обработкой ошибок
    match output.write_all(data) {
        Ok(_) => {
            println!("Successfully wrote {} bytes to '{}'", data.len(), filename);
            Ok(())
        }
        Err(e) => {
            eprintln!("Error writing to file '{}': {}", filename, e);
            Err(e)
        }
    }
}


/// Чтение бинарного файла
pub fn read_binary_file(filename: &str, data: &mut [u8]) -> io::Result<usize> {
    let mut input = File::open(filename)?;
    input.read(data)
}