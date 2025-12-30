run package:
    cd the-book/{{ package }} && cargo run

list:
    @echo "Available packages:"
    @ls -1 the-book/ | grep -v '^hello_world$' | sort

test package:
    cd the-book/{{ package }} && cargo test
