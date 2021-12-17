// Improvements: less repetitions, combine calculations of both tasks

#[derive(Debug, Clone)]
struct Position {
    x: i32,
    y: i32,
}

pub fn first() -> String {
  // target area: x=257..286, y=-101..-57
  let target_x = (257,286);
  let target_y = (-101,-57);

  let higest_y = find_highest_y(target_x, target_y);

  println!("reached target: {}", higest_y);
  higest_y.to_string()    
}

pub fn second() -> String {
  // target area: x=257..286, y=-101..-57
  let target_x = (257,286);
  let target_y = (-101,-57);

  let velocities = find_all_velocities(target_x, target_y);

  velocities.len().to_string()    
}

fn find_highest_y(target_x: (i32, i32), target_y: (i32, i32)) -> i32 {
  let mut max_y = 0;

  for ax in 0..target_x.1 * 2 {
      for ay in target_y.0..10000 {          
          let mut velocity = Position { x: ax, y: ay };
          let mut current_position = Position { x: 0, y: 0 };

          let mut reached_target = false;
          let mut local_max_y = max_y;

          loop {
              current_position.x += velocity.x;
              current_position.y += velocity.y;

              if current_position.y > local_max_y {
                  local_max_y = current_position.y;
              }

              if (target_x.0 <= current_position.x && current_position.x <= target_x.1) &&
                (target_y.0 <= current_position.y && current_position.y <= target_y.1) {
                  reached_target = true;
                  break;
              }
              else if current_position.x > target_x.1 || current_position.y < target_y.0 {
                  break;
              }

              if velocity.x > 0 {
                velocity.x -= 1;
              } else if velocity.x < 0 {
                velocity.x += 1;
              }

              velocity.y -= 1;
          }

          if reached_target {
              max_y = local_max_y;
          }
      }
  }

  max_y
}

fn find_all_velocities(target_x: (i32, i32), target_y: (i32, i32)) -> Vec<Position> {
  let mut max_y = 0;
  let mut velocities: Vec<Position> = Vec::new();

  for ax in 0..target_x.1 * 2 {
      for ay in target_y.0..10000 {          
          let mut velocity = Position { x: ax, y: ay };
          let mut current_position = Position { x: 0, y: 0 };

          let mut reached_target = false;
          let mut local_max_y = max_y;

          loop {
              current_position.x += velocity.x;
              current_position.y += velocity.y;

              if current_position.y > local_max_y {
                  local_max_y = current_position.y;
              }

              if (target_x.0 <= current_position.x && current_position.x <= target_x.1) &&
                (target_y.0 <= current_position.y && current_position.y <= target_y.1) {
                  reached_target = true;
                  break;
              }
              else if current_position.x > target_x.1 || current_position.y < target_y.0 {
                  break;
              }

              if velocity.x > 0 {
                velocity.x -= 1;
              } else if velocity.x < 0 {
                velocity.x += 1;
              }

              velocity.y -= 1;
          }

          if reached_target {
              velocities.push(velocity);
          }
      }
  }

  velocities
}