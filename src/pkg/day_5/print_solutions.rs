use super::read_input::read_input;

pub fn print_solutions() {
    let filename = "./src/pkg/day_5/test.txt";

    let crate_mover_9000_tops = solve_using_crate_mover_9000(filename);
    let crate_mover_9001_tops = solve_using_crate_mover_9001(filename);

    println!("Day 5:");
    println!("The tops of the stacks after all moves have been completed with the CrateMover 9000 will be {crate_mover_9000_tops}.");
    println!("The tops of the stacks after all moves have been completed with the CrateMover 9001 will be {crate_mover_9001_tops}.");
}

fn solve_using_crate_mover_9000(filename: &str) -> String {
    let (mut workspace, orders) = read_input(filename);

    for order in orders {
        workspace.apply_crate_mover_9000_order(order);
    }

    return workspace.get_tops();
}

fn solve_using_crate_mover_9001(filename: &str) -> String {
    let (mut workspace, orders) = read_input(filename);

    for order in orders {
        workspace.apply_crate_mover_9001_order(order);
    }

    return workspace.get_tops();
}
