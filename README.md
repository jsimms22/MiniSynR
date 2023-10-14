# MiniSynR
**Placeholder repo name. Project to explore Rust by attempting to implement an axon binary file type reader and build methods for analyzing electrophysiology data.

It may be beneficial to move the abf_reader program into a separate repo before I begin working on 

best explanation of abf breakdown so far:
https://github.com/swharden/AbfSharp/tree/main
https://swharden.com/pyabf/
https://github.com/swharden/pyABF

Someone else's Rust implementation:
https://crates.io/crates/rust_abf
>they attempt reading abf via direct memory allocation using memory mapping directly where we are using read methods. Might consider using their crate for the second part of this project if I get too busy to work on my own implementation.

discussion on memmap vs read (follow-up):
https://stackoverflow.com/questions/45972/mmap-vs-reading-blocks

libraries in use:
std::io
std::fs

Check out: (maybe):
https://cran.r-project.org/web/packages/readABF/index.html

Helpful comment on handling reading files into buffer:
https://www.reddit.com/r/rust/comments/dekpl5/how_to_read_binary_data_from_a_file_into_a_vecu8/

Guide to convert format characters in python to datatypes:
https://docs.python.org/3/library/struct.html

random:
https://github.com/Zabolekar/readABF
https://www.moleculardevices.com/sites/default/files/en/assets/user-guide/dd/cns/axon-binary-file-format-v2-0-9.pdf
https://www.moleculardevices.com/sites/default/files/en/assets/user-guide/dd/cns/axon-binary-file-format-v2-0-6.pdf