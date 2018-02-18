How to transfer json comming form iio stdout around:

start a netcat server listening for incomming data:

    nc -l -p 8080

connect to this netcat server pushing json data from iiopoll:

    ./iipoll -t 30 -l home | nc localhost 8080
