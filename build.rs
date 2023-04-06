extern crate embed_resource;

fn main() {
    embed_resource::compile("resources/icons.rc", embed_resource::NONE);
}
