mod cat;

fn main() -> anyhow::Result<()> {
    engine::run::<cat::Cat>()
}
