use super::{Dimension, CellularAutomata, Shape};

pub fn step(ca: &mut CellularAutomata) {
    let world_buffer = ca.world.clone();

    for (i, cell) in world_buffer.iter().enumerate() {
        let neighbours = match ca.dimension {
            Dimension::D1 => d1_neighbours(&world_buffer, i),
            Dimension::D2 => d2_neighbours(&world_buffer, i, &ca.shape),
            Dimension::D3 => d3_neighbours(&world_buffer, i),
        };

        if ca.rule.survival.contains(&neighbours) && cell == &1 {
            continue;
        } else if ca.rule.birth.contains(&neighbours) && cell == &0 {
            ca.world[i] = 1;
        } else {
            ca.world[i] = 0;
        }
    };
}

fn d1_neighbours(world: &[u8], index: usize) -> u8 {
    let mut count: u8 = 0;
    for x in 0..=2 {
        let i = index as i32 + (x - 1);
        if i != index as i32 && i >= 0 && i < world.len() as i32 {
            count += world[i as usize];
        }
    }
    count
}

fn d2_neighbours(world: &[u8], index: usize, size: &Shape) -> u8 { 
    let mut count: u8 = 0;

    // ###
    for mut x in 0..=2 {
        x -= 1;
        let i = index as i32 - x - size.x;
        let line_index = ((index - index % size.x as usize) / size.x as usize) as i32;
        let line_i = (i - i % size.x) / size.x;
        if i >= 0 && line_i == line_index - 1
        {
            count += world[i as usize];
        }
    }
    // #x#
    for mut x in 0..=2 {
        x -= 1;
        let i = index as i32 - x;
        let line_index = ((index - index % size.x as usize) / size.x as usize) as i32;
        let line_i = (i - i % size.x) / size.x;
        if i >= 0 && i != index as i32 && line_i == line_index
        {
            count += world[i as usize];
        }
    }

    // ###
    for mut x in 0..=2 {
        x -= 1;
        let i = index as i32 - x + size.x;
        let line_index = ((index - index % size.x as usize) / size.x as usize) as i32;
        let line_i = (i - i % size.x) / size.x;

        if i < world.len() as i32 && line_i == line_index + 1
        {
            count += world[i as usize];
        }
    }
    count 
}

fn d3_neighbours(world: &[u8], index: usize) -> u8 {
    todo!();
}