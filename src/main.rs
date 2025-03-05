trait FileSystemObject: std::fmt::Debug {
    fn get_name(&self) -> &str;
}

#[derive(Debug)]
struct FileObject {
    name: String,
    content: String,
}

#[derive(Debug)]
struct DirObject {
    name: String,
    content: Vec<Box<dyn FileSystemObject>>,
}

impl FileSystemObject for FileObject {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl FileSystemObject for DirObject {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl FileObject {
    fn get_content(&self) -> &str {
        &self.content // Возвращаем содержимое файла
    }
}

impl DirObject {
    fn get_content(&self) -> &Vec<Box<dyn FileSystemObject>> {
        &self.content // Возвращаем ссылку на содержимое директории
    }
}

fn main() {
    
}

// Добавляем трейт для приведения типа
