#!/bin/bash

# TODO: Port this to rust and integrate it in the unit tests.
if [[ $# != 1 ]]; then
    exit 1
fi

cd $1
for (( i = 0; i < 3; i++ )); do
    sensorroot="iio:device$i"
    mkdir $sensorroot
    echo "HANS" > $sensorroot/name
    echo 42 > $sensorroot/in_temp_input
    echo 50 > $sensorroot/in_humidityrelative_input
    if [[ $i == 2 ]]; then
        echo 666 > $sensorroot/in_pressure_input
    fi
done
