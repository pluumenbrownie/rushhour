use std::hash::Hasher;
use std::hash::Hash;
use std::sync::Arc;
use std::{fs, mem, iter::{repeat, successors}, ops::Deref};
use std::collections::hash_map::DefaultHasher;
use either::Either;
use smallvec::SmallVec;


#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct VehicleSegment {
    id: u8,
    direction: Direction,
    segments_left: u8,
}


impl VehicleSegment {
    pub fn new(id: String, direction: Direction, segments_left: u8) -> Result<VehicleSegment, ()> {
        let veh_id = VehicleSegment::string_to_veh_id(id).expect("String to id failed.");
        
        Ok(VehicleSegment {
            id: veh_id,
            direction,
            segments_left,
        })
    }

    fn string_to_veh_id(id: String) -> Result<u8, ()> {
        let id_as_bytes = id.as_bytes();
        let id1;
        let id2;
        if id_as_bytes.len() == 1 {
            if 65 <= id_as_bytes[0] && id_as_bytes[0] <= 90 {
                id1 = id_as_bytes[0] - 64;
                id2 = 0;
            } else {
                // eprintln!("Failed at len=1.");
                return Err(());
            }
        } else if id_as_bytes.len() == 2 {
            if 65 <= id_as_bytes[0] && id_as_bytes[0] <= 90 &&
               65 <= id_as_bytes[1] && id_as_bytes[1] <= 90 {
                id1 = id_as_bytes[1] - 64;
                id2 = (id_as_bytes[0] - 64) << 5;
            } else {
                // eprintln!("Failed at len=2.");
                return Err(());
            }
        } else {
            // eprintln!("Failed at other len.");
            return Err(());
        }
        
        // Ok(dbg!(dbg!(id1) + dbg!(id2)))
        Ok(id1 + id2)
    }

    pub fn id_string(&self) -> String {
        let letter1 = (self.id & 0b00011111) + 64;
        let mut letter2 = (self.id & 0b11100000) >> 5;
        if letter2 == 0 {
            letter2 = 32;
            String::from_utf8_lossy(&[letter1, letter2]).to_string()
        } else {
            letter2 += 64;
            String::from_utf8_lossy(&[letter2, letter1]).to_string()
        }
    }
}


#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Direction {
    Horizontal,
    Vertical,
}


impl Direction {
    fn from_str(dir_char: &str) -> Self {
        if dir_char == "H" {
            Self::Horizontal
        } else {
            Self::Vertical
        }
    }
}


#[derive(Default, Debug, Clone, Hash, Eq, PartialEq)]
pub enum Tile {
    Vehicle(VehicleSegment),
    #[default]
    Empty,
}
use Tile::{Empty, Vehicle};


impl Tile {
    const fn empty(&self) -> bool {
        match self {
            Self::Vehicle(_) => false,
            Self::Empty => true,
        }
    }
}


/// Get Moves by running the `possible_moves` method om your `Board`.
#[derive(Debug, Clone, Hash)]
pub struct Move {
    pub vehicle_id: u8,
    pub direction: i8,
}


impl Move {
    pub fn get_id_string(&self) -> String {
        let letter1 = (self.vehicle_id & 0b00011111) + 64;
        let mut letter2 = (self.vehicle_id & 0b1110000) >> 5;
        if letter2 == 0 {
            String::from_utf8_lossy(&[letter1]).to_string()
        } else {
            letter2 += 64;
            String::from_utf8_lossy(&[letter2, letter1]).to_string()
        }
    }
}


#[derive(Clone, Hash)]
struct LinkedHistory {
    last_move: Move,
    next_link: Arc<Option<LinkedHistory>>,
}


#[derive(Clone, Hash)]
pub struct Board {
    pub contents: SmallVec<[SmallVec<[Tile; 12]>; 12]>,
    previous: Arc<Option<LinkedHistory>>,
}


impl Board {
    pub fn new(size: u8) -> Self {
        let mut board_vecs = SmallVec::<[_; 12]>::with_capacity(size.into());

        for _ in 0..size {
            let mut new_col = SmallVec::<[_; 12]>::with_capacity(size.into());
            for _ in 0..size {
                new_col.push(Tile::Empty);
            }
            board_vecs.push(new_col);
        }
        
        Self {
            contents: board_vecs,
            previous: Arc::new(Option::None),
        }
    }
    pub fn show(&self) {
        let mut x_row = false;

        print!("┌");
        for _ in &self.contents {
            print!("───");
        }
        println!("┐");

        for row in &self.contents {
            print!("│");
            for tile in row {
                print!(" ");
                match tile {
                    Vehicle(vehicle) => {
                        print!("{:2}", vehicle.id_string());
                        if vehicle.id == VehicleSegment::string_to_veh_id("X".to_string()).expect("X conversion failed.") {x_row = true}
                    }, 
                    Empty => print!("  "), 
                }
            }
            println!("{}", if x_row { x_row = false; " =>"} 
                           else {"│"});
        }

        print!("└");
        for _ in &self.contents {
            print!("───");
        }
        println!("┘");
    }


    pub fn is_won(&self) -> Result<bool, ()> {
        let x_location = self.find_vehicle(VehicleSegment::string_to_veh_id("X".to_string())?)?;
        if self.contents.len().checked_sub(x_location.1).ok_or(())? == 2 {
            return Ok(true);
        }
        Ok(false)
    }


    pub fn get(&self, location: &(usize, usize)) -> &Tile {
        &self.contents[location.0][location.1]
    }


    fn take(&mut self, location: &(usize, usize)) -> Tile {
        mem::take(&mut self.contents[location.0][location.1])
    }


    /// location1.0 == location2.0
    fn swap_horizontal(&mut self, location1: &(usize, usize), location2: &(usize, usize)) {
        let row = self.contents.get_mut(location1.0)
            .expect("Row index out of bounds.");
        row.swap(location1.1, location2.1);
    }


    /// location1.1 == location2.1
    fn swap_vertical(&mut self, location1: &(usize, usize), location2: &(usize, usize)) {
        let (highest, lowest) = if location1.0 > location2.0 {
            (location1, location2)
        } else {
            (location2, location1)
        };
        let (left, right) = self.contents.split_at_mut(highest.0);
        let row1 = left.get_mut(lowest.0)
            .expect("Lowest row index out of bounds.");
        let row2 = right.get_mut(0)
            .expect("Highest row index out of bounds.");

        row1[lowest.1..=lowest.1].swap_with_slice(&mut row2[highest.1..=highest.1]);
    }
 

    pub fn fill(&mut self, file_path: &str) {
        let contents = fs::read_to_string(file_path)
            .expect("Reading file failed.");
        
        for line in contents.split('\n').skip(1) {
            // gameboard files end with a newline sometimes
            if line.is_empty() {break}

            self.add_vehicle(line);
        }
    }


    pub fn move_vehicle(&mut self, veh_move: Move) {
        let vehicle_location = self.find_vehicle(veh_move.vehicle_id)
            .expect("Vehicle not found.");

        // extract usefull information from given vehicle.
        let (direction, segments, origin) = 
            if let Vehicle(vehicle) = self.get(&vehicle_location) 
            {
                (vehicle.direction.clone(), 
                 vehicle.segments_left,
                 vehicle_location)
            } else {
                panic!("Board.find_vehicle returned an invalid tile (this shouldn't happen).")
            };

        // Order of swaps should depend on the direction of the move.
        let order = if veh_move.direction < 0 {
            Either::Left(0usize..=segments.into())
        } else {
            Either::Right((0usize..=segments.into()).rev())
        };
        
        for offset in order {
            // find tile to consider.
            let old_loc = match direction {
                Direction::Horizontal => (
                    origin.0,
                    origin.1.checked_add(offset)
                        .expect("checked_add_signed failed."),
                ),
                Direction::Vertical => (
                    origin.0.checked_add(offset)
                        .expect("checked_add_signed failed."),
                    origin.1,
                ),
            };
            // find new location of tile.
            let new_loc = match direction {
                Direction::Horizontal => (
                    old_loc.0,
                    old_loc.1.checked_add_signed(veh_move.direction.into())
                        .expect("checked_add_signed failed."),
                ),
                Direction::Vertical => (
                    old_loc.0.checked_add_signed(veh_move.direction.into())
                        .expect("checked_add_signed failed."),
                    old_loc.1,
                ),
            };
            
            match direction {
                Direction::Horizontal => self.swap_horizontal(&old_loc, &new_loc),
                Direction::Vertical => self.swap_vertical(&old_loc, &new_loc),
            };
        }

        let second_last = mem::take(&mut self.previous);
        let _ = mem::replace(&mut self.previous, Arc::new(Some(LinkedHistory{
                    last_move: veh_move,
                    next_link: second_last
                })));
    }


    /// Returns all possible moves for this board.
    /// 
    /// Iterates over the board. Once an empty tile is found, search in all 
    /// four directions for a vehicle which can move to that tile.
    pub fn possible_moves(&self) -> Result<Vec<Move>, ()> {
        let mut moves_vec: Vec<Move> = vec![];

        // iterates over locations of empty tiles
        for location in self.contents.iter()
            .enumerate()
            .flat_map(|(n, col)| repeat(n).zip(
                col.iter()
                .enumerate()
                .filter(|(_, x)| x.empty())
                .map(|t| t.0)
            ))
        {
            // horizontal right
            let candidate: Option<(&Tile, i8)> = self.contents.get(location.0)
                .unwrap()
                .iter()
                .skip(
                    location.1.checked_add(1)
                    .unwrap()
                )
                .zip((i8::MIN..=-1i8).rev())
                .find(|(tile, _)| !tile.empty());

            if let Some((Vehicle(vehicle), direction)) = candidate {
                match vehicle.direction {
                    Direction::Horizontal => moves_vec.push(Move { 
                        vehicle_id: vehicle.id, 
                        direction,
                    }),
                    Direction::Vertical => (),
                }
            }
            
            // horizontal left
            let candidate: Option<(&Tile, i8)> = self.contents.get(location.0)
                .unwrap()
                .iter()
                .take(location.1)
                .rev()
                .zip(1..)
                .find(|(tile, _)| !tile.empty());

            if let Some((Vehicle(vehicle), direction)) = candidate {
                match vehicle.direction {
                    Direction::Horizontal => moves_vec.push(Move { 
                        vehicle_id: vehicle.id, 
                        direction,
                    }),
                    Direction::Vertical => (),
                }
            }

            // vertical down 
            let candidate = self.contents.iter()
                .filter_map(|row| row.get(location.1)
                )
                .skip(location.0.checked_add(1).unwrap())
                .zip((i8::MIN..=-1i8).rev())
                .find(|(tile, _)| !tile.empty());

            if let Some((Vehicle(vehicle), direction)) = candidate {
                match vehicle.direction {
                    Direction::Vertical => moves_vec.push(Move { 
                        vehicle_id: vehicle.id, 
                        direction,
                    }),
                    Direction::Horizontal => (),
                }
            }

            // vertical up
            let candidate = self.contents.iter()
                .map(|row| row.get(location.1).unwrap())
                .take(location.0)
                .rev()
                .zip(1..)
                .find(|(tile, _)| !tile.empty());
            
            if let Some((Vehicle(vehicle), direction)) = candidate {
                match vehicle.direction {
                    Direction::Vertical => moves_vec.push(Move { 
                        vehicle_id: vehicle.id, 
                        direction,
                    }),
                    Direction::Horizontal => (),
                }
            }
        }
        if moves_vec.is_empty() {
            Err(())
        } else {
            Ok(moves_vec)
        }
    }


    /// Adds a vehicle to the board from a csv line.
    fn add_vehicle(&mut self, id_line: &str) {
        let mut vehicle_info = id_line.split(',');
        let veh_id  = vehicle_info.next().expect("veh_id parsing failed.");
        let veh_dir = vehicle_info.next().expect("veh_dir parsing failed.");

        // some variables must be converted.
        let mut veh_col: usize = vehicle_info.next()
            .expect("veh_col parsing failed.")
            .parse()
            .unwrap();
        let mut veh_row: usize = vehicle_info.next()
            .expect("veh_row parsing failed.")
            .parse()
            .unwrap();
        let mut veh_len: u8 = vehicle_info.next()
            .expect("veh_len parsing failed.")
            .parse()
            .unwrap();

        while veh_len > 0 {
            veh_len -= 1;
            // dbg!(veh_id, veh_dir, veh_len);
            self.contents[veh_row - 1][veh_col - 1] = Tile::Vehicle(VehicleSegment::new(veh_id.into(), Direction::from_str(veh_dir), veh_len).unwrap());
            //     VehicleSegment { 
            //         id: VehicleSegment::string_to_veh_id(veh_id), 
            //         direction: Direction::from_str(veh_dir), 
            //         segments_left: veh_len, 
            //     }
            // );

            if veh_dir == "H" {
                veh_col += 1;
            } else {
                veh_row += 1;
            }
        }
    }


    pub fn show_history(&self) {
        let turn_iterator = successors(
            self.previous.deref().as_ref(), 
            |p: &&LinkedHistory| p.next_link.deref().as_ref()
        );

        for turn in turn_iterator {
            println!("{:?}", turn.last_move);
        }
    }


    pub fn export(&self, file_path: &str) {
        let turn_iterator: Vec<&Move> = successors(
            self.previous.deref().as_ref(), 
            |p: &&LinkedHistory| p.next_link.deref().as_ref()
        ).map(|x| &x.last_move).collect();

        let mut writer = csv::Writer::from_path(file_path).expect("Making writer failed.");
        writer.write_record(["car", "move"]).expect("Writing heading failed.");

        for turn in turn_iterator.into_iter().rev() {
            writer.serialize((&turn.get_id_string(), turn.direction)).expect("Writing entry failed.");
        }
        
        writer.flush().expect("Flussing failed.");

    }


    pub fn get_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        // let mut s = Hasher::
        self.contents.hash(&mut s);
        s.finish()
    }


    fn find_vehicle(&self, id_number: u8) -> Result<(usize, usize), ()> {
        for numbered_row in self.contents.iter().enumerate() {
            let (row_num, row) = numbered_row;

            for numbered_tile in row.iter().enumerate() {
                let (col_num, tile) = numbered_tile;
                if let Vehicle(veh) = tile {
                    if veh.id == id_number {
                        return Result::Ok((row_num, col_num));
                    }
                }
            }
        }
        Result::Err(())
    }
}
