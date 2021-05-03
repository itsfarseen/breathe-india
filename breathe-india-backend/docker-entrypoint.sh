#!/bin/sh
# wait till postgres is up
while ! nc -z postgres 5432 </dev/null; do
    echo waiting for postgres to be up.
    sleep 5;
done
echo postgres is up. starting backend..
./breathe-india-backend
