use gitgud_greedy;

fn main() {
    let cli = gitgud_greedy::init();
    cli.print_info();
    cli.match_subcommand();
}
