//! Simulation for the car and racetrack
//!
//! There's a couple things to note here. The simulation is designed so that the directions from
//! the user happen less frequently individual state updates (referred to as "ticks"). The
//! simulation takes only a reference to a racetrack, with the only relevant state being the
//! attributes associated with the car.
//!
//! This module doesn't perform any interaction with user-submitted code. Currently-running
//! simulations are represented by the [`Simulation`] type, and are updated there.

use super::{Point, TICKS_PER_SECOND};
use pyo3::prelude::pyclass;
use serde::Serialize;
use std::collections::HashSet;
use std::f32::consts::PI;

/// In-memory representation of a racetrack
///
/// This type is generated with the [`Racetrack::from_str`] associated function.
///
/// The actual representation here is grid of [tiles](GridTile), where each tile records:
///  1. Whether it's inside the track; and
///  2. The way that the track border crosses through it, if applicable.
/// When we're determining information about the car's environment (and if it's collided with the
/// wall), splitting the racetrack into tiles helps us to limit the search space for lines that we
/// might have intersected with.
#[derive(Clone)]
pub struct Racetrack {
    /// The height, in tiles of the racetrack. Equal to `grid.len()`
    pub height: usize,
    /// The width, in tiles of the racetrack. Equal to `grid[i].len()` for all `i`
    pub width: usize,

    /// Rows of grid tiles. The tile at `grid[i][j]` covers the rectangle
    /// `(i*tile_size, j*tile_size)` to `((i + 1)*tile_size, (j + 1)*tile_size)`.
    pub grid: Vec<Vec<GridTile>>,

    /// The starting state of the car
    pub initial_car_state: Car,

    /// Two points defining the finish line. Individual tiles have a marker to indicate whether the
    /// finish line crosses them; this defines the *way* that that happens.
    pub finish_line: (Point, Point),

    /// The size of an individual tile
    pub tile_size: f32,

    /// The number of laps of the course that need to be performed to win
    pub laps: i32,
}

#[derive(Copy, Clone, Debug, Serialize)]
pub enum GridTile {
    /// A tile on the edge of the track. `border` gives two points that this edge of the
    /// racetrack passes through - the side of the border that's inside the racetrack can be
    /// inferred from the neighboring tiles
    ///
    /// For racetrack edges that exactly align with the edge of the tile, the tile with a `Border`
    /// variant is the one contained in the racetrack. A consequence of this is that there cannot
    /// be any racetracks with sections that are a single tile wide.
    ///
    /// All `Border` tiles were originally defined as part of the racetrack
    Border {
        border: (Point, Point),
        contains_finish_line: bool,
    },
    /// A tile fully contained within the racetrack, not bordering any edge.
    /// `contains_finish_line` indicates whether the finish line crosses through this tile in the
    /// grid
    Inside { contains_finish_line: bool },
    /// A tile fully outside of the racetrack, not bordering any edge
    Outside,
}

impl GridTile {
    /// Returns true iff the `GridTile` is the `Inside` variant
    pub fn is_inside(&self) -> bool {
        match self {
            Self::Inside { .. } => true,
            _ => false,
        }
    }

    /// Returns true iff the `GridTile` is the `Border` variant
    pub fn is_border(&self) -> bool {
        match self {
            Self::Border { .. } => true,
            _ => false,
        }
    }

    /// Returns true iff the `GridTile` is the `Outside` variant
    pub fn is_outside(&self) -> bool {
        match self {
            Self::Outside => true,
            _ => false,
        }
    }
}

// Note: the size of the car really only makes sense when compared to the size of the tiles in a
// racetrack grid. The size of the car is probably unlikely to change, whereas the tile size is
// explicitly variable.

/// The absolute size length of the car
pub const CAR_LENGTH: f32 = 1.0;
/// The width of the car
pub const CAR_WIDTH: f32 = 0.3;
/// The number of laps of the course required
pub const NUM_LAPS: i32 = 1;

/// All of the information about the car at a particular point in time
// TODO - added debug for ex result
#[pyclass]
#[derive(Copy, Clone, Serialize, Debug)]
pub struct Car {
    /// The position of the car
    pub pos: Point,
    /// The angle the car is facing, anticlockwise from the positive x direction - in radians
    pub angle: f32,
    /// The current speed, in "unit distance per simulation tick", of the car
    pub speed: f32,

    /// The maximum speed of the car in units per tick
    #[serde(skip)]
    pub max_speed: f32,
    //    /// The maximum acceleration of the car
    //    #[serde(skip)]
    //    pub max_acc: f32,
    //    /// The maximum deceleration of the car
    //    #[serde(skip)]
    //    pub max_dec: f32,
    // The maximum turning speed of the car in radians per tick
    #[serde(skip)]
    pub max_turn: f32,
}

// Car speed constants, in terms of the change per tick. As such, each parameter is first relative
// to the values PER SECOND, and then divides by the number of ticks in a second.
//
// Acceleration is additionally relative to the maximum speed.
const CAR_MAX_SPEED: f32 = 10.0 / TICKS_PER_SECOND as f32;
const CAR_MAX_ACC: f32 = 0.5 * CAR_MAX_SPEED / TICKS_PER_SECOND as f32;
const CAR_MAX_DEC: f32 = 0.3 * CAR_MAX_SPEED / TICKS_PER_SECOND as f32;
const CAR_MAX_TURNING_SPEED: f32 = 4.0 * PI / 3.0 / TICKS_PER_SECOND as f32;

impl Car {
    pub fn max_acc(&self) -> f32 {
        (1.0 - self.speed / self.max_speed) * CAR_MAX_ACC
    }

    pub fn max_dec(&self) -> f32 {
        (self.speed / self.max_speed) * CAR_MAX_DEC
    }

    pub fn pos_of_corners(&self) -> Vec<Point> {
        // Let's imagine the car going upwards; its angle will be PI/2. We want to determine where
        // the corners are. We can use the the front/back and left/right offset from one corner to
        // determine all the rest, so let's just focus on getting the position of the front-right
        // corner, relative to all the rest.
        //
        // So let's look at the following diagram of the car. The asterisk gives us the center of
        // the car:
        //          +---+ displacement from center to center right
        //          |   |
        //      +---+===+ -+
        //      |   |   |  | displacement from center to front center
        //      |   |   |  |
        //      |   |   |  |
        //      |   *   | -+
        //      |       |
        //      |       |
        //      |       |
        //      +-------+
        //
        // If we calculate these as `Point`s, we'll get the x and y components separately, so we
        // can add all the positive and negative combinations to get the four corners of the car.
        //
        // Getting these displacements is actualy pretty simple! Let's go through them. Looking at
        // these with the above diagram in mind makes quick checks nice for us :)
        let to_front = Point {
            x: self.angle.cos() * CAR_LENGTH,
            y: self.angle.sin() * CAR_LENGTH,
        };

        // The angle here is rotated clockwise by a quarter-turn -- i.e. PI/2. We could use that
        // directly, but we know from our trig identities that:
        //
        //   cos(θ - π/2) =   cos(π/2 - θ) =   sin(θ), and
        //   sin(θ - π/2) = - sin(π/2 - θ) = - cos(θ).
        //
        // So we get:
        let to_right = Point {
            x: self.angle.sin() * CAR_WIDTH,
            y: -self.angle.cos() * CAR_WIDTH,
        };

        vec![
            // Front right
            self.pos + to_front + to_right,
            // Front left
            self.pos + to_front - to_right,
            // Back right
            self.pos - to_front + to_right,
            // Back left
            self.pos - to_front - to_right,
        ]
    }
}

// Characters that represent the bounds of the racetrack
const OUT_OF_BOUNDS_CHAR: char = 'x';
const IN_BOUNDS_CHAR: char = ' ';
const START_TILE_CHAR: char = 's';
const FINISH_LINE_CHAR: char = '*';

/// The original representation of a tile, as it was parsed
#[derive(Copy, Clone)]
enum TileRepr {
    InBounds,
    OutOfBounds,
    Start,
    FinishLine,
}

impl TileRepr {
    /// Parses a `TileRepr` from a single character, if possible
    fn from_char(c: char) -> Option<Self> {
        match c {
            OUT_OF_BOUNDS_CHAR => Some(Self::OutOfBounds),
            IN_BOUNDS_CHAR => Some(Self::InBounds),
            START_TILE_CHAR => Some(Self::Start),
            FINISH_LINE_CHAR => Some(Self::FinishLine),
            _ => None,
        }
    }

    /// Returns whether the tile represents a regiont that's requested to be part of the racetrack
    fn is_part_of_racetrack(&self) -> bool {
        match self {
            Self::InBounds | Self::Start | Self::FinishLine => true,
            Self::OutOfBounds => false,
        }
    }
}

/// A representation of the grid description that we parse to create a `Racetrack`
struct InitialGrid {
    // The starting tile, given by its (x, y) indexes (so the actual tile is `rows[y][x]`).
    start_tile: (usize, usize),
    rows: Vec<Vec<TileRepr>>,
    width: usize,
}

impl InitialGrid {
    /// Parses the string into the grid of `TileRepr`s, to give the tiles that represent the grid
    /// description
    ///
    /// This is then used by [`Racetrack::make`] to actually construct the type.
    ///
    /// The returned tuple is `(width, grid)`, where all of the rows in `grid` have length equal to
    /// `width`. The ordering of the rows has already been reversed, so that `grid[0]` occurs at
    /// the bottom of the string.
    fn from_str(input: &str) -> Result<Self, String> {
        let mut chars = input.chars().peekable();

        // A helper macro to make the parsing a bit easier
        //
        // This can be thought of as exposing two function-like things:
        //   consume_next(c: char, err_fn: Fn() -> String)
        // and
        //   consume_next(s: &str, err_fn: Fn() -> String)
        //
        // The second variant is selected by the `@str` prefix within the macro call, and
        // internally just repeatedly calls the single-character version.
        //
        // If the character doesn't match, we return the error generated by `err_fn`. We're using a
        // function so that things like string formatting don't happen unless we actually need to
        // generate the error.
        macro_rules! consume_next {
            ($char:expr; $err_fn:expr) => {{
                if chars.next() != Some($char) {
                    return Err(($err_fn)().into());
                }
            }};
            (@str $s:expr; $err_fn:expr) => {{
                for c in $s.chars() {
                    consume_next!(c; $err_fn);
                }
            }};
        }

        consume_next!('+'; || "expected a starting '+'");
        let mut width = 0;
        while chars.peek() != Some(&'+') {
            consume_next!('-'; || "expected top row to contain dashes ('-')");
            width += 1;
        }
        // The first line should be followed by a newline
        consume_next!(@str "+\n"; || "top row should be terminated by '+\\n'");

        // The rows are initially constructed from top to bottom (by how they appear in the
        // string). We'll reverse this once we're done so that index 0 is at the bottom.
        let mut rows = Vec::new();
        let mut start_coords = None;

        while chars.peek() != Some(&'+') {
            // Each row is constructed by a beginning pipe ('|'), exactly `width` characters in
            // { 'x', ' ', 's', '*' }, and a trailing pipe.
            consume_next!('|'; || format!("expected row {} to start with '|'", rows.len() + 2));

            // Construct the row as we're parsing
            let mut row = Vec::with_capacity(width);
            for _ in 0..width {
                let tile_repr = chars.next().and_then(TileRepr::from_char).ok_or_else(|| {
                    format!(
                        "expected one of '{}', '{}', '{}', or '{}' at {:row$}:{:col$}",
                        OUT_OF_BOUNDS_CHAR,
                        IN_BOUNDS_CHAR,
                        START_TILE_CHAR,
                        FINISH_LINE_CHAR,
                        // Adding two means that these are indexed starting from 1
                        row = rows.len() + 2,
                        col = row.len() + 2,
                    )
                })?;

                if let TileRepr::Start = tile_repr {
                    match start_coords {
                        None => start_coords = Some((row.len(), rows.len())),
                        Some(_) => return Err("multiple start tiles found".to_owned()),
                    }
                }

                row.push(tile_repr);
            }
            rows.push(row);

            // Each *internal* row ends with a pipe and trailing newline
            consume_next!(@str "|\n"; || "expected trailing pipe ('|') at end of inner row");
        }
        // As promised above, we reverse the ordering of the rows
        rows.reverse();

        // We're expecting the bottom line to be the same as the top:
        // +2 for the '+' on each side
        consume_next!(@str &input[..width + 2]; || "expected bottom line to equal top");

        // The string should now either be empty, or have a trailing newline. If there's a newline,
        // we'll take it - and then we'll check that it's empty.
        if let Some('\n') = chars.peek() {
            chars.next();
        }

        if let Some(c) = chars.next() {
            return Err(format!("expected end of input, found character {:?}", c));
        }

        // Now that we've parsed the input, we'll offload constructing the type itself to a
        // dedicated function

        Ok(InitialGrid {
            rows,
            width,
            start_tile: start_coords.ok_or_else(|| "no start tile found")?,
        })
    }
}

impl Racetrack {
    /// Parses a `Racetrack` description from a string
    pub fn from_str(input: &str) -> Result<Self, String> {
        let init_grid = InitialGrid::from_str(input)?;

        // For now, we'll use the same tile size everywhere. There probably isn't an immediate need
        // to configure it on a course-by-course basis later; most maps will probably be a similar
        // size.
        let tile_size = 2.0_f32;
        Self::make(init_grid, tile_size)
    }

    /// Constructs the `Racetrack`, provided a representation of the grid the user described
    ///
    /// This function contains a pretty subjective subroutine; it performs all of the smoothing
    /// that we might need to do to make the walls of the racetrack nicer, which is entirely a
    /// "best effort" sort of implementation. The constraints on that are pretty loose, so it
    /// should be fairly possible to improve.
    fn make(initial_grid: InitialGrid, tile_size: f32) -> Result<Self, String> {
        // We start the car at the center of the tile, so we need to add 0.5 for its center
        let start_car_pos = Point {
            x: initial_grid.start_tile.0 as f32 * tile_size + 0.5 * tile_size,
            y: initial_grid.start_tile.1 as f32 * tile_size + 0.5 * tile_size,
        };

        let initial_car_state = Car {
            pos: start_car_pos,
            // Currently, the car will always start pointing downwards. This could be something
            // we'd like to configure in the future, but it's not necessary yet.
            angle: 3.0 * PI / 2.0,
            // The car always starts at a standstill - another thing that could be changed but
            // probably doesn't need to be
            speed: 0_f32,
            max_speed: CAR_MAX_SPEED,
            max_turn: CAR_MAX_TURNING_SPEED,
        };

        let width = initial_grid.width;
        let height = initial_grid.rows.len();
        let laps = NUM_LAPS;

        // It turns out to be useful to have some way of referring to directions. We'll use this at
        // a couple points later.
        #[rustfmt::skip]
        #[derive(Copy, Clone, PartialEq, Eq)]
        enum Direction { Up, Down, Left, Right }
        use Direction::*;

        // This gets a little tricky, because there's quite a few things that we'd like to verify
        // about our grids. In order, they are:
        //
        //  1. The racetrack is one contiguous region;
        //  2. The racetrack is never "too skinny";
        //  3. The finish line is "valid"; and
        //  4. If the car is on a "border" tile, it's on the side within the racetrack
        //
        // We'll go through these conditions one-by-one, with some more explanation in each of
        // those sections. We'll perform the smoothing between checking conditions 2 & 3.
        //
        // First up, condition 1:
        // We know that the car starts on a tile that's part of the racetrack (because
        // `TileRepr::Start` is part of the racetrack), so we'll use that as a starting point to
        // flood the graph of tiles corresponding to the racetrack.

        let mut is_part_of_racetrack = vec![vec![false; width]; height];
        let mut flood_stack = vec![initial_grid.start_tile];

        // Mark all of the tiles inside the track that are reachable from the starting tile
        while let Some((x, y)) = flood_stack.pop() {
            if y >= height || x >= width {
                continue;
            }

            let is_included = &mut is_part_of_racetrack[y][x];
            // If this "node" (grid tile) has already been marked as inside the track, we don't
            // need to repeat work.
            if *is_included {
                continue;
            }

            if initial_grid.rows[y][x].is_part_of_racetrack() {
                *is_included = true;

                // Continue to the other directions. `saturating_sub` here prevents underflows;
                // repeating values isn't really a problem, because we already check if they've
                // been set.
                flood_stack.push((x.saturating_sub(1), y));
                flood_stack.push((x, y.saturating_sub(1)));
                flood_stack.push((x + 1, y));
                flood_stack.push((x, y + 1));
            }
        }

        // If there are any tiles that *are* part of the racetrack, but aren't reachable from the
        // starting tile, that's an error.
        for y in 0..height {
            for x in 0..width {
                let repr = initial_grid.rows[y][x];
                if repr.is_part_of_racetrack() && !is_part_of_racetrack[y][x] {
                    return Err("some racetrack tiles are not reachable from the start".to_owned());
                }
            }
        }

        // Condition 2:
        //
        // This is a bit of a strange condition. It's essentially to account for a "limitation" of
        // the way that we represent the border tiles. Because borders can only have a single line
        // going through them, and their line MUST cut off any tiles that aren't part of the
        // racetrack, tiles cannot have opposite neighboring tiles that arne't part of the
        // racetrack. The full set of allowed and disallowed patterns is:
        //
        //               Allowed                 Disallowed
        //         ╔═══╗  ╔═══╗  ╔═══╗     ╔═══╗  ╔═══╗  ╔═══╗
        //         ║- -║  ║- -║  ║-x-║     ║- -║  ║-x-║  ║-x-║
        //         ║   ║  ║x  ║  ║x  ║     ║x x║  ║x x║  ║x x║
        //         ║- -║  ║- -║  ║- -║     ║- -║  ║- -║  ║-x-║
        //         ╚═══╝  ╚═══╝  ╚═══╝     ╚═══╝  ╚═══╝  ╚═══╝
        //
        // Where 'x' is for tiles that are not in the racetrack, ' ' is for tiles that are, and '-'
        // is for the tiles that we don't care about.
        //
        // For the purposes of this analysis, we don't care about the values of the tiles that
        // don't directly border the one we're concerned with.
        //
        // Finally, note that the edges of the encapsulating region count for "tiles" that aren't
        // part of the racetrack, even though there aren't actually tiles there.

        // The final produced grid for the racetrack, because we're doing the smoothing as we check
        // this condition.
        let mut grid = Vec::with_capacity(height);

        // While we're iterating over the entire grid, we'll store the tiles representing the
        // finish line for later, just so that we won't have to search for them.
        let mut finish_line_tiles: HashSet<(usize, usize)> = HashSet::new();

        for (y, row) in initial_grid.rows.iter().enumerate() {
            // The row to be added to the produced grid
            let mut grid_row = Vec::with_capacity(width);

            for (x, tile) in row.iter().enumerate() {
                // If this particular tile isn't inside the racetrack, we don't need to bother;
                // it's not relevant to the condition we're checking.
                if !tile.is_part_of_racetrack() {
                    grid_row.push(GridTile::Outside);
                    continue;
                }

                // Record if it's part of the finish line
                if let TileRepr::FinishLine = tile {
                    finish_line_tiles.insert((x, y));
                }

                // Determine the set of bordering tiles that are "outside" the racetrack
                let up_outside = (y == height) || !is_part_of_racetrack[y + 1][x];
                let right_outside = (x == width) || !is_part_of_racetrack[y][x + 1];
                let down_outside = (y == 0) || !is_part_of_racetrack[y - 1][x];
                let left_outside = (x == 0) || !is_part_of_racetrack[y][x - 1];

                // There's too many outside bordering tiles if either opposite pair has both tiles
                // outside:
                if (up_outside && down_outside) || (left_outside && right_outside) {
                    return Err(format!(
                        "racetrack is too skinny around tile at (x = {}, y = {})",
                        // +1 to the coordinates so that we continue the theme of error messages
                        // having indexes starting at 1
                        x + 1,
                        y + 1,
                    ));
                }

                // Now that we know we're looking at a valid tile within the racetrack, we'll set
                // it as entirely inside if it is. Otherwise, we'll continue.
                if !(up_outside || right_outside || down_outside || left_outside) {
                    // The `contains_finish_line` here defaults to false; we'll update it later
                    // when we construct the finish line.
                    grid_row.push(GridTile::Inside {
                        contains_finish_line: false,
                    });
                    continue;
                }

                // We're dealing with a border tile. There's two cases here, with eight in total
                // between them.

                // Turn the one or two outside booleans that are true into the direction they
                // correspond to:
                let mut n_border_outside = if up_outside { 1 } else { 0 };
                let mut most_clockwise = Up;

                let direction_pairs = &[
                    (Right, right_outside),
                    (Down, down_outside),
                    (Left, left_outside),
                ];

                for (d, b) in direction_pairs {
                    if *b {
                        most_clockwise = *d;
                        n_border_outside += 1;
                    }
                }

                // The bottom-left corner
                let bot_left = Point {
                    x: x as f32 * tile_size,
                    y: y as f32 * tile_size,
                };
                let bot_right = bot_left.add_x(tile_size);
                let top_left = bot_left.add_y(tile_size);
                let top_right = top_left.add_x(tile_size);

                // If we have only one bordering tile outside the racetrack, it must be at the
                // direction given by `most_clockwise`

                let border_points = if n_border_outside == 1 {
                    match most_clockwise {
                        Up => (top_left, top_right),
                        Down => (bot_left, bot_right),
                        Left => (bot_left, top_left),
                        Right => (bot_right, top_right),
                    }
                } else {
                    // When there's two bordering tiles that are "outside", the border line is
                    // opposite corners.
                    match most_clockwise {
                        // Covering left & up
                        Up => (bot_left, top_right),
                        // Covering up & right
                        Right => (top_left, bot_right),
                        // Covering right & down
                        Down => (top_right, bot_left),
                        // Covering down & left
                        Left => (bot_right, top_left),
                    }
                };

                grid_row.push(GridTile::Border {
                    border: border_points,
                    contains_finish_line: false,
                });
            }

            grid.push(grid_row);
        }

        // --- TODO: Smoothing ---
        //
        // This actually isn't implemented yet! There's already *some* that exists purely because
        // of the constraints of `GridTile::Border`, but there should be enough information here to
        // eventually do it in a way that makes sense. The tracking issue for this is here:
        //   https://github.com/RiverNewbury/AIRacingGame/issues/1

        // Constraint 3:
        //
        // The ideal implementation here would allow the finish line to be flexible, such that we
        // generate the it as the only line that passes through all of the finish line tiles (and
        // no additional ones). We'd take the finish line tiles as something like:
        //    ╔═══════╗    ╔═══╗
        //    ║**     ║    ║*  ║
        //    ║  ***  ║ or ║ * ║
        //    ║     **║    ║  *║
        //    ╚═══════╝    ╚═══╝
        // This sort of thing is *really* hard though, so we aren't doing that yet.
        // (TODO: Proper finish line)
        //
        // Currently, we just take the finish line as a horizontal line going through the middle of
        // the starting tile, corresponding to how the car starts facing upwards.

        let start_row = initial_grid.start_tile.1;
        let start_col = initial_grid.start_tile.0;

        // The pair of points that define the finish line. For now, this is just the middle of the
        // left and right sides of the starting tile.
        let finish_line = (
            start_car_pos.add_x(-0.5 * tile_size),
            start_car_pos.add_x(0.5 * tile_size),
        );

        // We need to make sure that all of the finish-line tiles in the graph are accounted for in
        // this area of the graph.
        let mut num_finish_tiles_accounted_for = 0;

        // Go left of the starting position:
        for tile in grid[start_row][..start_col].iter_mut().rev() {
            match tile {
                GridTile::Border {
                    contains_finish_line,
                    ..
                }
                | GridTile::Inside {
                    contains_finish_line,
                } => *contains_finish_line = true,
                // We're only going as far as the border of the track in this direction; if we
                // simply `continue`d, it would be possible to count the finish line on both sides
                // of a loop -- something we don't want to do.
                GridTile::Outside => break,
            }

            num_finish_tiles_accounted_for += 1;
        }

        // And then go to the right:
        for tile in grid[start_row][start_col + 1..].iter_mut() {
            #[rustfmt::skip]
            match tile {
                GridTile::Border { contains_finish_line, ..  }
                | GridTile::Inside { contains_finish_line } => *contains_finish_line = true,
                GridTile::Outside => break,
            };

            num_finish_tiles_accounted_for += 1;
        }

        // Finally, check that all of the finish line tiles in the parsed representation were
        // accounted for in the horizontal line here.
        if num_finish_tiles_accounted_for != finish_line_tiles.len() {
            return Err(
                "malformed finish line; should span the track horizontally from the start tile"
                    .to_owned(),
            );
        }

        // And then we're done! We just need to return the final `Racetrack`:
        Ok(Racetrack {
            height,
            width,
            grid,
            initial_car_state,
            finish_line,
            tile_size,
            laps,
        })
    }

    /// Produces a copy of the tile containing the given point
    ///
    /// ## Panics
    ///
    /// Panics if the point is outside the range of the bounds of the racetrack
    pub fn get_tile(&self, mut p: Point) -> GridTile {
        p /= self.tile_size;
        self.grid[p.y as usize][p.x as usize]
    }
}
