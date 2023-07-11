use crate::hamming_encoding::*;

type Disk = Vec<Bit>;

pub struct RaidII {
    parity_bit_disk: Disk,
    data_bit_disk: Vec<Disk>,
    hamming_bit_disks: Vec<Disk>,
    total_disks: usize,
    disk_size: usize,
    free_space: usize,
    files: Vec<File>
}

pub struct File {
    name: String,
    start_pos: usize,
    end_pos: usize,
    size: usize,
    file_type: FileType
}

pub enum FileType {
    Text
}

pub enum WriteResult{
    Success,
    NotEnoughSpace
}

pub enum ReadResult {
    NotFound,
    Success(FileType, Vec<u8>)
}

impl RaidII {
    pub fn from_data_capacity(disk_size: usize) -> Self {
        let mut hamming_disks = 0;
        let capacity = 8;
        while (capacity + hamming_disks + 1) > (1 << hamming_disks) {
            hamming_disks += 1;
        }

        let mut data_bit_disk = Vec::with_capacity(capacity);
        let mut hamming_bit_disks = Vec::with_capacity(hamming_disks);
        let files = Vec::new();

        for _ in 0..capacity {
            data_bit_disk.push(vec![]);
        }

        for _ in 0..hamming_disks {
            hamming_bit_disks.push(vec![]);
        }

        RaidII {
            parity_bit_disk: Vec::new(),
            data_bit_disk,
            hamming_bit_disks,
            total_disks: 1 + capacity + hamming_disks,
            disk_size,
            free_space: disk_size,
            files
        }
    }

    pub fn write_file(&mut self, data: &Vec<u8>, file_type: FileType, name: String) -> WriteResult {
        match file_type {
            FileType::Text => {
                if self.free_space > data.len() {
                    for byte in data {
                        self.write_byte(*byte)
                    }

                    let file = File {
                        name,
                        start_pos: self.disk_size - self.free_space,
                        end_pos: self.disk_size - self.free_space + data.len(),
                        size: data.len(),
                        file_type,
                    };

                    self.files.push(file);
                    return WriteResult::Success
                }
                else {
                    return WriteResult::NotEnoughSpace
                }
            }
        }
    }

    fn write_byte(&mut self, byte: u8) {
        let bits = &bit_vector_from_bytes(&vec![byte]);
        let encoded_bits = encode(&bits);
        self.parity_bit_disk.push(encoded_bits[0]);

        for i in 1..self.data_bit_disk.len() {
            self.data_bit_disk[i].push(encoded_bits[i]);
        }

        for i in 1..self.data_bit_disk.len() {
            self.data_bit_disk[i].push(encoded_bits[i]);
        }

        for i in (self.hamming_bit_disks.len())..(self.total_disks) {
            self.hamming_bit_disks[i].push(encoded_bits[i]);
        }
    }

    pub fn read_file(&mut self, name: String ) -> ReadResult {
        match self.files.iter().find(|x| x.name == name) {
            Some(file) => {
                unimplemented!()
            },
            None => return ReadResult::NotFound,
        }
    }

}
