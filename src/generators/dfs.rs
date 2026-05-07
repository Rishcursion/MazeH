pub enum GenAlgorithm {
    Dfs,
    Ellers,
    Sidewinder,
    Prims,
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    north: bool,
    south: bool,
    west: bool,
    east: bool,
}

#[derive(Debug, Clone)]
pub struct Maze {
    grid: Vec<Cell>,
    height: usize,
    width: usize,
}

impl Maze {
    /// Constructor function for initializing a flat 1D maze with height*width Cells
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            grid: vec![
                Cell {
                    north: true,
                    south: true,
                    west: true,
                    east: true
                };
                width * height
            ],
            height,
            width,
        }
    }
    /// Helper function for getting the value of a cell at position [x, y] in the maze
    pub fn cell(&self, x: usize, y: usize) -> Option<Cell> {
        if x >= self.width || y >= self.height {
            Some(self.grid[y * self.width + x])
        } else {
            None
        }
    }
    /// Helper function for getting a mutable cell at position [x, y] in the maze for carving
    pub fn mut_cell(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        if x >= self.width || y >= self.height {
            Some(&mut self.grid[y * self.width + x])
        } else {
            None
        }
    }
}

impl GenAlgorithm {
    /// Function to generate a maze using recursive backtracker or DFS Algorithm
    pub fn gen_dfs(height: usize, width: usize) {
        let mut maze = Maze::new(height, width);
    }
    pub fn carve_maze(maze: &mut Maze, start_x: usize, start_y: usize) {}
}

impl Direction {
    pub fn all() -> [Direction; 4] {
        [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ]
    }

    pub fn opposite(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    pub fn delta(self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        }
    }
}
