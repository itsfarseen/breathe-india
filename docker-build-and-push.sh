#!/bin/bash
# Configuration, change as needed
SSH_USER=farzeen
SSH_HOST=bisket-magenta

docker-compose build

# -M put into master mode so that it can be controlled through a socket even if
# put in background.
# -S specify the control socket
# -f put in background.
# -N Don't run any commands, just forward.
# -T Don't allocate tty
ssh -M -S ./ssh-tunnel-ctrl-socket -fNT -L 5000:localhost:5000 $SSH_USER@$SSH_HOST

docker-compose push

# Stop ssh tunnelling
ssh -S ./ssh-tunnel-ctrl-socket -O exit $SSH_USER@$SSH_HOST
