# Author: Jonathan Samson

cat << 'EOF'
------------------------------------------------------
|  $$\   $$\  $$$$$$\ $$$$$$$$\ $$$$$$$$\  $$$$$$\   |
|  $$$\  $$ |$$  __$$\\__$$  __|$$  _____|$$  __$$\  |
|  $$$$\ $$ |$$ /  $$ |  $$ |   $$ |      $$ /  \__| |
|  $$ $$\$$ |$$ |  $$ |  $$ |   $$$$$\    \$$$$$$\   |
|  $$ \$$$$ |$$ |  $$ |  $$ |   $$  __|    \____$$\  |
|  $$ |\$$$ |$$ |  $$ |  $$ |   $$ |      $$\   $$ | |
|  $$ | \$$ | $$$$$$  |  $$ |   $$$$$$$$\ \$$$$$$  | |
|  \__|  \__| \______/   \__|   \________| \______/  |
|                                                    |
| 1) install pre-commit hooks                        |
| 2) compile                                         |
| 3) run                                             |
------------------------------------------------------

EOF

# get command from user
if [[ -z $1 ]]; then
  read -p 'enter command number: ' cmd
else
  cmd=$1
fi

# execute script based on user command
case $cmd in
  1)
    pre-commit install
    pre-commit install --hook-type commit-msg
    ;;
  2)
    cargo build
    ;;
  3)
    cargo run
    ;;
  *)
    echo "command '$cmd' not found"
    ;;
esac