use crate::hamming_encoding::*;

type Disk = Vec<Bit>;

pub struct RaidII {
    // Disks
    parity_bit_disk: Disk,
    data_bit_disks: Vec<Disk>,
    hamming_bit_disks: Vec<Disk>,

    total_disks: usize,
    total_capcity: usize,
    disk_size: usize,
    free_space: usize,
    files: Vec<File>,
}

pub struct File {
    name: String,
    start_pos: usize,
    end_pos: usize,
    size: usize,
    file_type: FileType,
}

#[derive(Copy, Clone)]
pub enum FileType {
    Text,
}

pub enum FileWriteResult {
    Success,
    NotEnoughSpace,
}

pub enum FileReadResult {
    NotFound,
    DisksCorrupted,
    Success(FileType, Vec<u8>),
}

enum ReadData<T> {
    Some(T),
    CorruptedData,
}

impl RaidII {
    pub fn from_data_capacity(disk_size: usize) -> Self {
        let mut hamming_disks = 0;
        let capacity = 8;
        while (capacity + hamming_disks + 1) > (1 << hamming_disks) {
            hamming_disks += 1;
        }

        let mut data_bit_disks = Vec::with_capacity(capacity);
        let mut hamming_bit_disks = Vec::with_capacity(hamming_disks);
        let files = Vec::new();

        for _ in 0..capacity {
            data_bit_disks.push(vec![]);
        }

        for _ in 0..hamming_disks {
            hamming_bit_disks.push(vec![]);
        }

        let total_disks = 1 + capacity + hamming_disks;

        RaidII {
            parity_bit_disk: Vec::new(),
            data_bit_disks,
            hamming_bit_disks,
            total_disks,
            disk_size,
            free_space: disk_size * total_disks,
            files,
            total_capcity: disk_size * total_disks,
        }
    }

    pub fn write_file(
        &mut self,
        data: &Vec<u8>,
        file_type: &FileType,
        name: &String,
    ) -> FileWriteResult {
        match file_type {
            FileType::Text => {
                if self.free_space > data.len() {
                    for byte in data {
                        self.write_byte(*byte)
                    }

                    let file = File {
                        name: name.clone(),
                        start_pos: self.total_capcity - self.free_space,
                        end_pos: self.total_capcity - self.free_space + data.len(),
                        size: data.len(),
                        file_type: *file_type,
                    };

                    self.free_space -= data.len();
                    self.files.push(file);

                    FileWriteResult::Success
                } else {
                    FileWriteResult::NotEnoughSpace
                }
            }
        }
    }

    fn write_byte(&mut self, byte: u8) {
        let bits = &bit_vector_from_bytes(&vec![byte]);
        let mut written_bit_counter = 0;
        let encoded_bits = encode(&bits);
        self.parity_bit_disk.push(encoded_bits[written_bit_counter]);
        written_bit_counter += 1;

        for i in 0..self.data_bit_disks.len() {
            self.data_bit_disks[i].push(encoded_bits[written_bit_counter]);
            written_bit_counter += 1;
        }

        for i in 0..self.hamming_bit_disks.len() {
            self.hamming_bit_disks[i].push(encoded_bits[written_bit_counter]);
            written_bit_counter += 1;
        }
    }

    pub fn read_file(&mut self, name: &String) -> FileReadResult {
        let mut corrupted_data = false;
        match self.files.iter().find(|x| (*x).name == *name) {
            Some(file) => {
                let mut bytes = Vec::with_capacity(file.size);
                for position in file.start_pos..file.end_pos {
                    match self.read_byte(position) {
                        ReadData::Some(byte) => bytes.push(byte),
                        ReadData::CorruptedData => {
                            corrupted_data = true;
                            break;
                        }
                    }
                }

                if corrupted_data {
                    FileReadResult::NotFound
                } else {
                    FileReadResult::Success(file.file_type, bytes)
                }
            }
            None => FileReadResult::NotFound,
        }
    }

    fn read_byte(&self, position: usize) -> ReadData<u8> {
        let mut bits = Vec::with_capacity(self.total_disks);
        bits.push(self.parity_bit_disk[position]);

        for i in 0..self.data_bit_disks.len() {
            bits.push(self.data_bit_disks[i][position]);
        }

        for i in 0..self.hamming_bit_disks.len() {
            bits.push(self.hamming_bit_disks[i][position]);
        }

        match decode(&mut bits) {
            HammingDecodeResult::NoError { decoded_bits } => Self::analyze_bits(&decoded_bits),
            HammingDecodeResult::OneError { decoded_bits, .. } => Self::analyze_bits(&decoded_bits),
            HammingDecodeResult::DoubleError => ReadData::CorruptedData,
        }
    }

    fn analyze_bits(data: &Vec<Bit>) -> ReadData<u8> {
        let bytes = bit_vector_to_bytes(&data);
        if bytes.len() == 1 {
            ReadData::Some(bytes[0])
        } else {
            ReadData::CorruptedData
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::raid::*;

    #[test]
    fn raid_create_test() {
        let bytes_per_disk = 1024;
        let raid_ii = RaidII::from_data_capacity(bytes_per_disk);
        assert_eq!(raid_ii.disk_size, bytes_per_disk);
        assert_eq!(raid_ii.files.len(), 0);
        assert_eq!(raid_ii.data_bit_disks.len(), 8);
        assert_eq!(raid_ii.hamming_bit_disks.len(), 4);
        assert_eq!(raid_ii.total_disks, 13);
        assert_eq!(raid_ii.free_space, raid_ii.disk_size * raid_ii.total_disks);
        assert_eq!(
            raid_ii.total_capcity,
            raid_ii.disk_size * raid_ii.total_disks
        );
        assert_eq!(raid_ii.files.len(), 0);
    }

    #[test]
    fn raid_use_test1() {
        let bytes_per_disk = 1024;
        let mut raid_ii = RaidII::from_data_capacity(bytes_per_disk);
        let text_data = "Hello, Rust!";
        let bytes = text_data.as_bytes().to_vec();
        let file_name = "Greeting".to_owned();
        let file_type = FileType::Text;

        match raid_ii.write_file(&bytes, &file_type, &file_name) {
            FileWriteResult::Success => match raid_ii.read_file(&file_name) {
                FileReadResult::NotFound => assert!(false),
                FileReadResult::DisksCorrupted => assert!(false),
                FileReadResult::Success(find_file_type, find_bytes) => match find_file_type {
                    FileType::Text => assert_eq!(bytes, find_bytes),
                },
            },
            FileWriteResult::NotEnoughSpace => assert!(false),
        }
    }

    #[test]
    fn raid_use_test2() {
        let bytes_per_disk = 1024;
        let mut raid_ii = RaidII::from_data_capacity(bytes_per_disk);
        let text_data = "Rust is ideal for many people for a variety of reasons. 
        Rust is for people who crave speed and stability in a language. 
        By speed, we mean both how quickly Rust code can run and the speed at which Rust lets you write programs. 
        The Rust compiler’s checks ensure stability through feature additions and refactoring.";

        let bytes = text_data.as_bytes().to_vec();
        let file_name = "Introduction to Rust".to_owned();
        let file_type = FileType::Text;

        match raid_ii.write_file(&bytes, &file_type, &file_name) {
            FileWriteResult::Success => match raid_ii.read_file(&file_name) {
                FileReadResult::NotFound => assert!(false),
                FileReadResult::DisksCorrupted => assert!(false),
                FileReadResult::Success(find_file_type, find_bytes) => match find_file_type {
                    FileType::Text => assert_eq!(bytes, find_bytes),
                },
            },
            FileWriteResult::NotEnoughSpace => assert!(false),
        }
    }
}
