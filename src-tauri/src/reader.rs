use std::io::{Read, Seek, SeekFrom};
use std::sync::{Arc, Mutex};
use std::fs;

use tokio::sync::mpsc;

// pub const MAX_CONCURRENCY: usize = 8;
pub const SLICE_SIZE: usize = 1024 * 1024 * 25;
pub const CLUSTER_CAP: usize = 10;
pub const CLUSTER_SIZE: usize = SLICE_SIZE * CLUSTER_CAP;
pub const BUFFER_SIZE: usize = 1024 * 1024 * 4;

type File = Arc<Mutex<fs::File>>;

pub struct Reader {
  file: File,
  slices: usize,
  pub tx: mpsc::Sender<usize>,
  pub clusters: usize,
  pub size: usize,
}

impl Reader {
  pub fn new(path: &str, tx: mpsc::Sender<usize>) -> Self {
    let file = fs::File::open(path).unwrap();
    let size = fs::metadata(path).unwrap().len() as usize;

    let slices = (size + SLICE_SIZE - 1) / SLICE_SIZE;
    let clusters = (slices + CLUSTER_CAP - 1) / CLUSTER_CAP;

    Self {
      file: Arc::new(Mutex::new(file)),
      slices,
      tx,
      clusters,
      size,
    }
  }

  pub fn next_cluster(&self, idx: usize) -> Option<Cluster> {
    if idx == self.clusters {
      return None;
    }

    let tail = self.slices - idx * CLUSTER_CAP;
    let offset = idx * CLUSTER_SIZE;
    let cluster = Cluster {
      file: self.file.clone(),
      size: self.size,
      offset,
      slices: std::cmp::min(CLUSTER_CAP, tail),
      current_slice: 0,
      tx: self.tx.clone(),
    };

    Some(cluster)
  }
}

pub struct Cluster {
  file: File,
  size: usize,
  offset: usize,
  slices: usize,
  current_slice: usize,
  tx: mpsc::Sender<usize>,
}

impl Cluster {
  pub fn next_slice(&mut self) -> Option<Slice> {
    if self.current_slice == self.slices {
      return None;
    }

    let slice = Slice {
      file: self.file.clone(),
      size: self.size,
      offset: self.offset + self.current_slice * SLICE_SIZE,
      read: 0,
      tx: self.tx.clone(),
    };

    self.current_slice += 1;
    Some(slice)
  }
}

pub struct Slice {
  file: File,
  size: usize,
  offset: usize,
  read: usize,
  tx: mpsc::Sender<usize>,
}

impl Iterator for Slice {
    type Item = Result<Vec<u8>, std::io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
      if self.read == SLICE_SIZE {
        return None;
      }

      let size = std::cmp::min(SLICE_SIZE - self.read, BUFFER_SIZE);
      let size = std::cmp::min(size, self.size - self.read - self.offset);
      let mut buffer = vec![0; size];

      let mut file = self.file.lock().unwrap();
      file.seek(SeekFrom::Start((self.offset + self.read) as u64)).unwrap();
      let read = file.read(&mut buffer).unwrap();

      drop(file);
      if read == 0 {
        return None;
      }

      self.read += read;
      let tx = self.tx.clone();
      tokio::spawn(async move {
        tx.send(read).await.unwrap();
      });

      Some(Ok(buffer))
    }
  }