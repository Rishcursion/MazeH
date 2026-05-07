use rand::{rng, seq::SliceRandom};

/// Defintion for a cell in a maze, with cell state true representing a wall, and false representing open in the respective direction
#[derive(Debug, Clone, Copy)]
pub struct Cell {
    north: bool,
    south: bool,
    east: bool,
    west: bool,
}
/// Definition of the maze itself, with metadata and the state of the maze
#[derive(Debug, Clone)]
pub struct Maze {
    grid: Vec<Cell>,
    height: usize,
    width: usize,
}
#[derive(Debug, Clone, Copy)]
/// Enum for valid directions that can be checked, can be extended to NE,NW, SE,SW for more complexity in the future
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Cell {
    /// Constructor function for the initial cell state with all walls set
    pub fn new() -> Self {
        Self {
            north: true,
            south: true,
            east: true,
            west: true,
        }
    }

    /// Helper function to clear the wall in a particular direction
    pub fn unset_wall(&mut self, direction: Direction) {
        match direction {
            Direction::North => self.north = false,
            Direction::West => self.west = false,
            Direction::East => self.east = false,
            Direction::South => self.south = false,
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::new()
    }
}

impl Direction {
    /// Returns a fixed array of all directions
    pub fn all() -> [Direction; 4] {
        [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ]
    }
    /// Returns the opposite direction of the current direction
    pub fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }
    /// Returns the delta values to move in the specified direction
    pub fn delta(&self) -> (isize, isize) {
        match self {
            Self::North => (0, -1),
            Self::East => (1, 0),
            Self::South => (0, 1),
            Self::West => (-1, 0),
        }
    }
}

/// Maze-specific function definitions
impl Maze {
    /// Constructor for initializing a new maze with a defined height and width
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            grid: vec![Cell::default(); height * width],
            height,
            width,
        }
    }
    /// Returns a cell present in the grid, by translating from 2d coordinates to a 1d coordinate
    pub fn get_cell(&self, x: usize, y: usize) -> Option<Cell> {
        if (x < self.width) && (y < self.height) {
            Some(self.grid[y * self.width + x])
        } else {
            None
        }
    }

    /// Returns the 1d coord based on the 2d coords for the list representation of the maze
    pub fn get_1d_coord(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
    /// Returns coords of the next neighbour in a specified direction from the current cell
    pub fn neighbour(&self, x: usize, y: usize, direction: Direction) -> Option<(usize, usize)> {
        let (dx, dy) = direction.delta();
        let (new_x, new_y) = (
            usize::try_from(x as isize + dx).ok()?,
            usize::try_from(y as isize + dy).ok()?,
        );
        (new_x < self.width && new_y < self.height).then_some((new_x, new_y))
    }
    /// Connects a cell to another one in the specified direction, by removing the wall between them if it exists
    pub fn connect_cells(
        &mut self,
        curr_x: usize,
        curr_y: usize,
        direction: Direction,
    ) -> Option<(usize, usize)> {
        let (new_x, new_y) = self.neighbour(curr_x, curr_y, direction)?;
        let curr_idx = self.get_1d_coord(curr_x, curr_y);
        let new_idx = self.get_1d_coord(new_x, new_y);

        self.grid[curr_idx].unset_wall(direction);
        self.grid[new_idx].unset_wall(direction.opposite());

        Some((new_x, new_y))
    }

    /// Function for rendering the maze using ASCII
    pub fn render(&self) -> Option<()> {
        let repeated = "_".repeat(self.width * 2 - 1);
        println!(" {repeated}");
        for y in 0..self.height {
            print!("|");
            for x in 0..self.width {
                if !self.get_cell(x, y)?.south {
                    print!(" ");
                } else {
                    print!("_");
                }
                if !self.get_cell(x, y)?.east {
                    if self.get_cell(x, y)?.south && self.get_cell(x + 1, y)?.south {
                        print!("_");
                    } else {
                        print!(" ");
                    }
                } else {
                    print!("|");
                }
            }
            println!();
        }
        Some(())
    }
}

/// Wrapper function for kicking off the recursive function
pub fn gen_dfs(maze: &mut Maze) {
    let mut cells_visited: Vec<bool> = vec![false; maze.width * maze.height];
    dfs_helper(maze, 0, 0, &mut cells_visited);
}
/// Recursive in-place function to carve out a maze using the recursive backtracker algorithm
pub fn dfs_helper(maze: &mut Maze, x: usize, y: usize, cells_visited: &mut Vec<bool>) {
    let mut directions = Direction::all();

    cells_visited[maze.get_1d_coord(x, y)] = true;
    directions.shuffle(&mut rng());

    for direction in directions {
        let Some((nx, ny)) = maze.neighbour(x, y, direction) else {
            continue;
        };
        let n_idx = maze.get_1d_coord(nx, ny);
        if !cells_visited[n_idx] {
            cells_visited[n_idx] = true;
            maze.connect_cells(x, y, direction);
            dfs_helper(maze, nx, ny, cells_visited);
        }
    }
}

pub fn gen_ellers(maze: &mut Maze) {}
