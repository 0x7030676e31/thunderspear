use std::cmp::{max, min};
use std::mem;

#[derive(Default, Clone)]
struct GrowingHashmapMapElemChar<ValueType> {
  key: u32,
  value: ValueType,
}

struct GrowingHashmapChar<ValueType> {
  used: i32,
  fill: i32,
  mask: i32,
  map: Option<Vec<GrowingHashmapMapElemChar<ValueType>>>,
}

impl<ValueType> Default for GrowingHashmapChar<ValueType>
where
  ValueType: Default + Clone + Eq,
{
  fn default() -> Self {
    Self {
      used: 0,
      fill: 0,
      mask: -1,
      map: None,
    }
  }
}

impl<ValueType> GrowingHashmapChar<ValueType>
where
  ValueType: Default + Clone + Eq + Copy,
{
  fn get(&self, key: u32) -> ValueType {
    self.map
      .as_ref()
      .map_or_else(|| Default::default(), |map| map[self.lookup(key)].value)
  }

  fn get_mut(&mut self, key: u32) -> &mut ValueType {
    if self.map.is_none() {
      self.allocate();
    }

    let mut i = self.lookup(key);
    if self
      .map
      .as_ref()
      .expect("map should have been created above")[i]
      .value
      == Default::default()
    {
      self.fill += 1;
      // resize when 2/3 full
      if self.fill * 3 >= (self.mask + 1) * 2 {
        self.grow((self.used + 1) * 2);
        i = self.lookup(key);
      }

      self.used += 1;
    }

    let elem = &mut self
      .map
      .as_mut()
      .expect("map should have been created above")[i];
    elem.key = key;
    &mut elem.value
  }

  fn allocate(&mut self) {
    self.mask = 8 - 1;
    self.map = Some(vec![GrowingHashmapMapElemChar::default(); 8]);
  }

  /// lookup key inside the hashmap using a similar collision resolution
  /// strategy to `CPython` and `Ruby`
  fn lookup(&self, key: u32) -> usize {
    let hash = key;
    let mut i = hash as usize & self.mask as usize;

    let map = self
      .map
      .as_ref()
      .expect("callers have to ensure map is allocated");

    if map[i].value == Default::default() || map[i].key == key {
      return i;
    }

    let mut perturb = key;
    loop {
      i = (i * 5 + perturb as usize + 1) & self.mask as usize;

      if map[i].value == Default::default() || map[i].key == key {
          return i;
      }

      perturb >>= 5;
    }
  }

  fn grow(&mut self, min_used: i32) {
    let mut new_size = self.mask + 1;
    while new_size <= min_used {
      new_size <<= 1;
    }

    self.fill = self.used;
    self.mask = new_size - 1;

    let old_map = std::mem::replace(
      self.map
        .as_mut()
        .expect("callers have to ensure map is allocated"),
    vec![GrowingHashmapMapElemChar::<ValueType>::default(); new_size as usize],
    );

    for elem in old_map {
      if elem.value != Default::default() {
        let j = self.lookup(elem.key);
        let new_elem = &mut self.map.as_mut().expect("map created above")[j];
        new_elem.key = elem.key;
        new_elem.value = elem.value;
        self.used -= 1;
        if self.used == 0 {
          break;
        }
      }
    }

    self.used = self.fill;
  }
}

struct HybridGrowingHashmapChar<ValueType> {
  map: GrowingHashmapChar<ValueType>,
  extended_ascii: [ValueType; 256],
}

impl<ValueType> HybridGrowingHashmapChar<ValueType>
where
  ValueType: Default + Clone + Copy + Eq,
{
  fn get(&self, key: char) -> ValueType {
    let value = key as u32;
    if value <= 255 {
      let val_u8 = u8::try_from(value).expect("we check the bounds above");
      self.extended_ascii[usize::from(val_u8)]
    } else {
      self.map.get(value)
    }
  }

  fn get_mut(&mut self, key: char) -> &mut ValueType {
    let value = key as u32;
    if value <= 255 {
      let val_u8 = u8::try_from(value).expect("we check the bounds above");
      &mut self.extended_ascii[usize::from(val_u8)]
    } else {
      self.map.get_mut(value)
    }
  }
}

impl<ValueType> Default for HybridGrowingHashmapChar<ValueType>
where
  ValueType: Default + Clone + Copy + Eq,
{
  fn default() -> Self {
    HybridGrowingHashmapChar {
      map: GrowingHashmapChar::default(),
      extended_ascii: [Default::default(); 256],
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct RowId {
  val: isize,
}

impl Default for RowId {
  fn default() -> Self {
    Self { val: -1 }
  }
}

fn damerau_levenshtein_impl<Iter1, Iter2>(s1: Iter1, len1: usize, s2: Iter2, len2: usize) -> usize
where
  Iter1: Iterator<Item = char> + Clone,
  Iter2: Iterator<Item = char> + Clone,
{
  let max_val = max(len1, len2) as isize + 1;

  let mut last_row_id = HybridGrowingHashmapChar::<RowId>::default();

  let size = len2 + 2;
  let mut fr = vec![max_val; size];
  let mut r1 = vec![max_val; size];
  let mut r: Vec<isize> = (max_val..max_val + 1)
    .chain(0..(size - 1) as isize)
    .collect();

  for (i, ch1) in s1.enumerate().map(|(i, ch1)| (i + 1, ch1)) {
    mem::swap(&mut r, &mut r1);
    let mut last_col_id: isize = -1;
    let mut last_i2l1 = r[1];
    r[1] = i as isize;
    let mut t = max_val;

    for (j, ch2) in s2.clone().enumerate().map(|(j, ch2)| (j + 1, ch2)) {
      let diag = r1[j] + isize::from(ch1 != ch2);
      let left = r[j] + 1;
      let up = r1[j + 1] + 1;
      let mut temp = min(diag, min(left, up));

      if ch1 == ch2 {
        last_col_id = j as isize; // last occurence of s1_i
        fr[j + 1] = r1[j - 1]; // save H_k-1,j-2
        t = last_i2l1; // save H_i-2,l-1
      } else {
        let k = last_row_id.get(ch2).val;
        let l = last_col_id;

        if j as isize - l == 1 {
          let transpose = fr[j + 1] + (i as isize - k);
          temp = min(temp, transpose);
        } else if i as isize - k == 1 {
          let transpose = t + (j as isize - l);
          temp = min(temp, transpose);
        }
      }

      last_i2l1 = r[j + 1];
      r[j + 1] = temp;
    }
    last_row_id.get_mut(ch1).val = i as isize;
  }

  r[len2 + 1] as usize
}

#[allow(dead_code)]
pub fn damerau_levenshtein(a: &str, b: &str) -> usize {
  damerau_levenshtein_impl(a.chars(), a.chars().count(), b.chars(), b.chars().count())
}
