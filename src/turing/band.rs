use std::{collections::HashMap, fmt::{Write}};

// 

pub enum BandDirection {
    RIGHT,
    LEFT
}

// 

pub struct BandState<'a> {
    cursor: i32,
    left: i32,
    right: i32,
    data: Vec<&'a str>
}

impl<'a> BandState<'a> {

    pub fn toString(&self) -> &str {
        let mut data_string = String::new();

        for d in self.data() {
            data_string.write_str(d);
            data_string.write_str(",");
        }

        let result = format!("{{ cursor: {}, left: {}, right: {}, data: \"{}\" }}", self.cursor(), self.left(), self.right(), data_string);
        return Box::leak(result.into_boxed_str());
    }

    pub fn new(cursor: i32, left: i32, right: i32, data: Vec<&'a str>) -> Box<Self> {
        let instance = Box::new(Self {
            cursor,
            left,
            right,
            data
        });

        return instance;
    }

    // 

    pub fn cursor(&self) -> i32 { self.cursor }
    
    pub fn left(&self) -> i32 { self.left }
    pub fn right(&self) -> i32 { self.right }
    
    pub fn data(&self) -> Vec<&'a str> { self.data.to_owned() }

}

// 

pub struct Band<'a> {
    cursor: i32,
    band: HashMap<i32, &'a str>
}

impl<'a> Band<'a> {

    pub const BLANK_SYMBOL: &'a str = "0";

    pub fn from(data: Vec<&'a str>) -> Box<Self> {
        let mut instance = Self::new();
        
        let mut i = 0;
        
        for symbol in data {
            instance.band.insert(i, symbol);

            i+=1;
        }

        return instance;
    }

    // 

    pub fn new() -> Box<Self> {
        let instance = Box::new(Self {
            cursor: 0,
            band: HashMap::new()
        });

        return instance;
    }

    // 

    pub fn getCursor(&self) -> i32 { self.cursor }

    pub fn setCursor(&mut self, pos: i32) { self.cursor = pos; }
    
    // 

    pub fn moveCursor(&mut self, direction: BandDirection) -> i32 {
        match direction {
            BandDirection::LEFT => self.setCursor(self.getCursor() - 1),
            BandDirection::RIGHT => self.setCursor(self.getCursor() + 1)
        }

        return self.getCursor();
    }

    // 

    pub fn readAt(&self, pos: i32) -> &'a str {
        match self.band.get(&pos) {
            Some(&x) => x,
            None => return Band::BLANK_SYMBOL
        }
    }

    pub fn readCursor(&self) -> &'a str {
        return self.readAt(self.getCursor());
    }

    // 

    pub fn writeAt(&mut self, pos: i32, symbol: &'a str) {
        if self.band.contains_key(&pos) {
            self.band.remove(&pos);
        }

        self.band.insert(pos + 0, symbol);
    }

    pub fn writeCursor(&mut self, symbol: &'a str) {
        self.writeAt(self.getCursor() + 0, symbol);
    }

    //

    pub fn state(&self) -> Option<Box<BandState>> {
        let mut left: i32 = 0;
        let mut is_left_initialized = false;

        let mut right: i32 = 0;
        let mut is_right_initialized = false;

        let mut data: Vec<&'a str> = Vec::new();

        for _k in self.band.keys() {
            let k: i32 = _k + 0;
            
            if !is_left_initialized || left > k {
                left = k;
                is_left_initialized = true;
            }
            else if !is_right_initialized || right < k {
                right = k;
                is_right_initialized = true;
            }

            match self.band.get(_k) {
                Some(&x) => { data.push(x); println!("// // // // | {}", x); },
                None => { data.push(Band::BLANK_SYMBOL); println!("// // // // | 0"); }
            }
        }

        return Some(BandState::new(self.getCursor(), left, right, data));
    }

}