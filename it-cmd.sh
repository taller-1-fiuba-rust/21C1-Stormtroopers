#!/bin/bash

LOG=/tmp/it_log

HOST_REDIS="127.0.0.1 8081"

### Variables testing ###
RES_OK="127.0.0.1:8081> OK"
RES_NIL="127.0.0.1:8081> (nil)"
RES_0="127.0.0.1:8081> 0"
RES_1="127.0.0.1:8081> 1"
RES_2="127.0.0.1:8081> 2"
RES_3="127.0.0.1:8081> 3"
RES_12="127.0.0.1:8081> 12"
RES_4="127.0.0.1:8081> 4"
RES_TESTB="127.0.0.1:8081> testb"

#Grupo Server
TEST0="info"
#Grupo String & Keys
TEST4="set a 1"
TEST5="get a"
TEST6="get b"
TEST7="append a 2"
TEST8='get a'
TEST9="dbsize"
TEST10="flushdb"
TEST11="dbsize"
TEST12='set b testb'
TEST13="get b"
TEST14="del b"

TEST15="exists a"
TEST16="exists notexist"

TEST17="expire a 2"
TEST18="ttl a"
TEST19="get a"


#Grupo Set

#Grupo List

### Funciones ###

function test(){
	i=$1
  #test found
	line=$2
  #test expected
	to_test=$3
  #test name
	test_name=$4

	PURPLE='\033[0;33m'
	RED='\033[0;31m'
	GREEN='\033[0;32m'
	NC='\033[0m' # No Color
	
	if [[ $line == $to_test ]]; then
		printf "$test_name > $line >>>>>> TEST $i ... ${GREEN}ok${NC}\n\n"
  	else
  		printf "$test_name > $line >>>>>> TEST $i ... ${RED}FAILED${NC}. ${PURPLE}Expected $to_test, found $line ${NC}\n\n"
  	fi
}

### Main ###
#echo "Load Redis server ..."
#./target/debug/proyecto_taller_1 &
#sleep 2
#echo

echo "Exec it tests ..."

{ 
	echo $TEST4;
	echo $TEST5;
	echo $TEST6;
	echo $TEST7;
	echo $TEST8;
	echo $TEST9;
	echo $TEST10;
	echo $TEST11;
  echo $TEST12;
  echo $TEST13;
  echo $TEST14;
  echo $TEST4;
  echo $TEST15;
  echo $TEST16;
  echo $TEST17;
  echo $TEST18;
  echo $TEST19;

  #Necesario para cerrar la conexion y que la misma no quede colgada.
  echo "exit"
	sleep 1;
} | telnet $HOST_REDIS 1>$LOG 2>&1

sleep 1;
i=1
while IFS= read -r line
do

  line=$(echo "$line")
    if [ $i == "3" ] || [ $i = "2" ] || [ $i == "1" ]; then
  	 echo "Reading: "$line
    elif [[ $i == "4" ]]; then
  	test $i "$line" "$RES_OK" "$TEST4"
    elif [[ $i == "5" ]]; then
  	test $i "$line" "$RES_1" "$TEST5"
    elif [[ $i == "6" ]]; then
  	test $i "$line" "$RES_NIL" "$TEST6"
    elif [[ $i == "7" ]]; then
  	test $i "$line" "$RES_2" "$TEST7"
    elif [[ $i == "8" ]]; then
  	test $i "$line" "$RES_12" "$TEST8"
    elif [[ $i == "9" ]]; then
  	test $i "$line" "$RES_1" "$TEST9"
    elif [[ $i == "10" ]]; then
  	test $i "$line" "$RES_OK" "$TEST10"
    elif [[ $i == "11" ]]; then
  	test $i "$line" "$RES_0" "$TEST11"
    elif [[ $i == "12" ]]; then
    test $i "$line" "$RES_OK" "$TEST12"
    elif [[ $i == "13" ]]; then
    test $i "$line" "$RES_TESTB" "$TEST13"
    elif [[ $i == "15" ]]; then
    test $i "$line" "$RES_OK" "$TEST4"
    elif [[ $i == "16" ]]; then
    test $i "$line" "$RES_2" "$TEST15"
    elif [[ $i == "17" ]]; then
    test $i "$line" "$RES_0" "$TEST16"
    elif [[ $i == "18" ]]; then
    test $i "$line" "$RES_OK" "$TEST17"
    elif [[ $i == "19" ]]; then
    sleep 1
    test $i "$line" "$RES_1" "$TEST18"
    elif [[ $i == "20" ]]; then
    sleep 2
    test $i "$line" "$RES_NIL" "$TEST19"
    fi

  i=$((i+1))

done < $LOG

TEST15="exists a"
TEST16="exists notexist"

TEST17="expire a 2"
TEST18="ttl a"
TEST19="get a"

echo "Exec it tests Grupo Server"
rm $LOG
{ 
  echo $TEST0;
  #Necesario para cerrar la conexion y que la misma no quede colgada.
  echo "exit"
  sleep 1;
} | telnet $HOST_REDIS 1>$LOG 2>&1

sleep 1;

#-> Config Server:
#0) "verbose": "false"
#1) "timeout": "600"
#2) "server": "127.0.0.1"
#3) "port": "8081"
#4) "dbfilename": "dump.rdb"
#5) "sharing_count": "2"
#6) "logfile": "redis.log"

i=1
while IFS= read -r line
do
  #echo "in"
    line=$(echo "$line")
    if [ $i == "4" ] || [ $i == "3" ] || [ $i = "2" ] || [ $i == "1" ]; then
    echo "Reading: "$line
    elif [[ $i == "5" ]]; then
    test $i "$line" "-> connected_clients: 1" "$TEST0"
    elif [[ $i == "6" ]]; then
    test $i "$line" "-> tcp_port: 127.0.0.1:8081" "$TEST0"
    elif [[ $i == "8" ]]; then
    test $i "$line" "-> max_clients: 10" "$TEST0"
    elif [[ $i == "10" ]]; then
    test $i "$line" "-> uptime_days: 0" "$TEST0"
    elif [[ $i == "11" ]]; then
    test $i "$line" "-> actives_threads: 2" "$TEST0"
    elif [[ $i == "12" ]]; then
    test $i "$line" "-> Config Server:" "$TEST0"
    elif [[ $i == "13" ]]; then
    test $i "$line" '0) "verbose": "false"' "$TEST0"
    elif [[ $i == "14" ]]; then
    test $i "$line" '0) "verbose": "false"' "$TEST0"
    elif [[ $i == "15" ]]; then
    test $i "$line" '2) "server": "127.0.0.1"' "$TEST0"
    elif [[ $i == "16" ]]; then
    test $i "$line" '3) "port": "8081"' "$TEST0"
    
    fi
  i=$((i+1))

done < $LOG


echo "Exec it tests Grupo Server"
rm $LOG
{ 
  echo $TEST0;
  #Necesario para cerrar la conexion y que la misma no quede colgada.
  echo "exit"
  sleep 1;
} | telnet $HOST_REDIS 1>$LOG 2>&1

sleep 1;


#list_test=("$TEST4" "$TEST5")
#j=4
#for test in "${list_test[@]}"; do
#  test $j "$line" "RES" 
#  j=$((j+1))  
#done


echo "Exit it test"