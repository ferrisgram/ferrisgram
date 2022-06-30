@echo off
echo Running FerrisGram Generator....
cd generator
cargo run
echo Formatting Code....
cd ..
cargo fmt
echo Successfully generated the wrapper!