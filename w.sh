
while true; do
    clear
    if [ $1 = "test" ]
    then
        cargo test
    else
        cargo run
    fi

    change=$(inotifywait -e close_write,moved_to,create src/)
    clear
    echo $change changed
done