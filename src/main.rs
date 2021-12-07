use gitgud_greedy;

fn main() {
    let cli = gitgud_greedy::Cli::new();
    cli.print();
    cli.match_subcommand();
}
