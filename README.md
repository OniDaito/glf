# glf

A rust crate for reading GLF Files from [Tritech Sonar](https://www.tritech.co.uk/).

## Documentation

Documentation is available at [https://docs.rs/glf/0.2.0/glf/](https://docs.rs/glf/0.2.0/glf/).

## Building

To build this crate, run:

    cargo build

## Usage

    use std::path::Path;
    use glf::GLF;
    
    let glf = GLF::new(Path::new("./pytritech_testdata/test_tritech.glf")).unwrap();
    println!("GLF Image 0: {}", glf.images[0].header.time);
    let img = glf.extract_image(1).unwrap();
    img.save("test.png").unwrap();

## Testing

To test the crate, you'll need to download a submodule that contains the test data. It's a little large and so isn't included in the basic install. To perform a full checkout of this repository you can run:

    git clone --recurse-submodules https://github.com/onidaito/glf

Or, if you've already checked out

    git submodule update --init --recursive

From then on, one can run the usual cargo command:

    cargo test