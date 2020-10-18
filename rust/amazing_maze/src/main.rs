use std::io::*;
use std::str::FromStr;
use std::collections::VecDeque;


struct Agent {
    pos: (usize, usize),
    queue: VecDeque<(usize, usize)>,
    route: Vec<Vec<i32>>,
}

impl Agent {
    fn new(width: usize, height: usize) -> Self {
        let pos = (0, 0);
        let mut queue = VecDeque::new();
        let mut route: Vec<_> = (0..height).map(|_| vec![0; width]).collect();

        route[0][0] = 1;
        queue.push_back(pos);

        Agent {
            pos: pos,
            queue: queue,
            route: route,
        }
    }

    fn set_pos(&mut self, pos: (usize, usize)) {
        self.pos = pos;
    }

    fn is_walkable(&self, dx: i32, dy: i32, h_wall: &Vec<Vec<i32>>, v_wall: &Vec<Vec<i32>>) -> bool {
        let y = self.pos.0;
        let x = self.pos.1;

        (dy == 1 && h_wall[y + 1][x] == 0)
        || (dy == -1 && h_wall[y][x] == 0)
        || (dx == 1 && v_wall[y][x + 1] == 0)
        || (dx == -1 && v_wall[y][x] == 0)
    }

    fn walk(&mut self, dx: i32, dy: i32) {
        let y = self.pos.0;
        let x = self.pos.1;
        let walked_y = (y as i32 + dy) as usize;
        let walked_x = (x as i32 + dx) as usize;

        if self.route[walked_y][walked_x] == 0 {
            self.queue.push_back((walked_y, walked_x));
            self.route[walked_y][walked_x] = self.route[y][x] + 1;
        }
    }
}

struct Maze {
    h_wall: Vec<Vec<i32>>,
    v_wall: Vec<Vec<i32>>,
}

impl Maze {
    fn new(width: usize, height: usize) -> Self {
        let wall = Maze::read_wall(width, height);
        Maze {
            h_wall: wall.0,
            v_wall: wall.1,
        }
    }
    fn read_wall(width: usize, height: usize) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
        let mut h_wall = Vec::new();
        let mut v_wall = Vec::new();

        h_wall.push(vec![1; width]);
        for i in 0..2 * height - 1 {
            if i % 2 == 0 {
                let mut line = read_line(width - 1);
                line.insert(0, 1);
                line.push(1);
                v_wall.push(line);
            } else {
                h_wall.push(read_line(width));
            }
        }
        h_wall.push(vec![1; width]);

        (h_wall, v_wall)
    }
}

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

fn read_line(len: usize) -> Vec<i32> {
    (0..len).map(|_| read::<i32>()).collect()
}

fn main() {
    loop {
        let width: usize = read();
        let height: usize = read();

        if width == 0 && height == 0 {
            break;
        }

        let mut agent = Agent::new(width, height);
        let maze = Maze::new(width, height);
        let direction: [[i32; 2]; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];

        while !agent.queue.is_empty() {
            let pos = agent.queue.pop_front().unwrap();
            agent.set_pos(pos);
            for direct in direction.iter() {
                if agent.is_walkable(direct[0], direct[1], &maze.h_wall, &maze.v_wall) {
                    agent.walk(direct[0], direct[1]);
                }
            }
        }
        println!("{:?}", &agent.route[height - 1][width - 1]);
    }
}