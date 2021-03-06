#!/bin/bash

# A script to start backend and frontend dev servers.
# This doesn't start the postgres server. 
# That you will have to set up and start manually, and put the connection URL in
# ./breathe-india-backend/.env

if [ -z "$TMUX" ]; then
    tmux new-session bash $0
    exit;
fi;

backend() {
    cd ./breathe-india-backend
    while [ $? -eq 0 ]; do
        cargo run;
        echo "Respawning.. Press Ctrl-C to abort.";
        sleep 1s;
    done;

}

frontend() {
    cd ./breathe-india-frontend
    while [ $? -eq 0 ]; do
        yarn dev;
        echo "Respawning.. Press Ctrl-C to abort.";
        sleep 1s;
    done;
}

if [ "$1" = "backend" ]; then
    backend;
elif [ "$1" = "frontend" ]; then
    frontend;
else
    tmux split bash $0 backend
    tmux split bash $0 frontend
fi;
