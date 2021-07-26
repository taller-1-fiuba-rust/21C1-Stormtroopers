#!/bin/bash

LOG=/tmp/it_log
LOG_SRV=/tmp/it_log_srv

HOST_REDIS="127.0.0.1 8081"
RES_HOST_REDIS="127.0.0.1:8081> "
### Variables testing ###
RES_OK="127.0.0.1:8081> OK"
RES_NIL="127.0.0.1:8081> (nil)"
RES_EMPTY_SET_LIST="(empty list or set)"

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

echo "#############################"
echo "### Load Redis server ... ###"
echo "#############################"
./target/debug/proyecto_taller_1 > /dev/null 2>&1 &
sleep 2
echo
pid_redis=$(pidof proyecto_taller_1)
echo "PID redis_server: ${pid_redis}"


### Main ###  
echo "###############"
echo "### flushdb ###"
echo "###############"
{
  echo $TEST10
  echo "exit"
  echo "exit"
  sleep 1;
} | telnet $HOST_REDIS 1>$LOG 2>&1
echo

echo "#########################################"
echo "### Exec it tests Grupo Keys & Server ###"
echo "#########################################"
rm $LOG

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
  #echo $TEST19;
  #echo $TEST19;
  #echo $TEST18;
  #echo $TEST19;
  #Necesario para cerrar la conexion y que la misma no quede colgada.
  echo "exit"
	sleep 1;
} | telnet $HOST_REDIS 1>$LOG 2>&1

sleep 1;
i=1
while IFS= read -r line
do

  line=$(echo "$line")
    #if [ $i == "3" ] || [ $i = "2" ] || [ $i == "1" ]; then
  	#echo "Reading: "$line
    if [[ $i == "4" ]]; then
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
    test $i "$line" "$RES_1" "$TEST15"
    elif [[ $i == "17" ]]; then
    test $i "$line" "$RES_0" "$TEST16"
    elif [[ $i == "18" ]]; then
    test $i "$line" "$RES_OK" "$TEST17"
    elif [[ $i == "19" ]]; then
    test $i "$line" "$RES_2" "$TEST18"
    elif [[ $i == "20" ]]; then
    test $i "$line" "$RES_1" "$TEST19"
    #elif [[ $i == "21" ]]; then
    #test $i "$line" "$RES_NIL" "$TEST19"
    #elif [[ $i == "22" ]]; then
    #test $i "$line" "$RES_NIL" "$TEST19"
    #elif [[ $i == "23" ]]; then
    #test $i "$line" "$RES_1" "$TEST18"
    #elif [[ $i == "24" ]]; then
    #test $i "$line" "$RES_NIL" "$TEST19"
    fi

  i=$((i+1))

done < $LOG


TEST15="exists a"
TEST16="exists notexist"

TEST17="expire a 2"
TEST18="ttl a"
TEST19="get a"
echo "##################################"
echo "### Exec it tests Grupo Server ###"
echo "##################################"
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
    #if [ $i == "4" ] || [ $i == "3" ] || [ $i = "2" ] || [ $i == "1" ]; then
    #echo "Reading: "$line
    if [[ $i == "5" ]]; then
    test $i "$line" "-> connected_clients: 1" "$TEST0"
    elif [[ $i == "6" ]]; then
    test $i "$line" "-> tcp_port: 127.0.0.1:8081" "$TEST0"
    elif [[ $i == "8" ]]; then
    test $i "$line" "-> max_clients: 10" "$TEST0"
    elif [[ $i == "10" ]]; then
    test $i "$line" "-> uptime_days: 0" "$TEST0"
    elif [[ $i == "11" ]]; then
    test $i "$line" "-> actives_threads: 2" "$TEST0"

    #Comento estas lineas porque el orden es arbitrario

    #elif [[ $i == "12" ]]; then
    #test $i "$line" "-> Config Server:" "$TEST0"
    #elif [[ $i == "13" ]]; then
    #test $i "$line" '0) "server": "127.0.0.1"' "$TEST0"
    #elif [[ $i == "17" ]]; then
    #test $i "$line" '0) "verbose": "false"' "$TEST0"
    #elif [[ $i == "15" ]]; then
    #test $i "$line" '2) "dbfilename": "dump.rdb"' "$TEST0"
    #elif [[ $i == "16" ]]; then
    #test $i "$line" '3) "port": "8081"' "$TEST0"
    
    fi
  i=$((i+1))

done < $LOG

echo "###############################"
echo "### Exec it tests Grupo Set ###"
echo "###############################"

TEST0="sadd s 1 2 3"
TEST01="scard s"
TEST1="sismember s 1"
TEST2="sismember s 2"
TEST3="sismember s 4"
TEST4="smembers s"
TEST5="srem s a"
TEST6="srem s 1"
TEST7="srem s 2 3"
TEST8="smembers s"

rm $LOG
{ 
  echo $TEST0;
  echo $TEST01;
  echo $TEST1;
  echo $TEST2;
  echo $TEST3;
  echo $TEST4;
  echo $TEST5;
  echo $TEST6;
  echo $TEST7;
  echo $TEST8;
  #Necesario para cerrar la conexion y que la misma no quede colgada.
  echo "exit"
  echo "exit"
  sleep 1;
} | telnet $HOST_REDIS 1>$LOG 2>&1

sleep 1;

i=1
while IFS= read -r line
do
  #echo "in"
    line=$(echo "$line")
    #if [ $i == "3" ] || [ $i = "2" ] || [ $i == "1" ]; then
    #echo "Reading: "$line
    if [[ $i == "4" ]]; then
    test $i "$line" "${RES_HOST_REDIS}3" "$TEST0"
    elif [[ $i == "5" ]]; then
    test $i "$line" "${RES_HOST_REDIS}3" "$TEST01"
    elif [[ $i == "6" ]]; then
    test $i "$line" "${RES_HOST_REDIS}1" "$TEST1"
    elif [[ $i == "7" ]]; then
    test $i "$line" "${RES_HOST_REDIS}1" "$TEST2"
    elif [[ $i == "8" ]]; then
    test $i "$line" "${RES_HOST_REDIS}0" "$TEST3"
    elif [[ $i == "9" ]]; then
    test $i "$line" "${RES_HOST_REDIS}0) 1" "$TEST4"
    elif [[ $i == "10" ]]; then
    test $i "$line" "1) 2" "$TEST4"
    elif [[ $i == "11" ]]; then
    test $i "$line" "2) 3" "$TEST4"
    elif [[ $i == "12" ]]; then
    test $i "$line" "${RES_HOST_REDIS}0" "$TEST5"
    elif [[ $i == "13" ]]; then
    test $i "$line" "${RES_HOST_REDIS}1" "$TEST6"
    elif [[ $i == "14" ]]; then
    test $i "$line" "${RES_HOST_REDIS}2" "$TEST7"
    elif [[ $i == "15" ]]; then
    test $i "$line" "${RES_HOST_REDIS}${RES_EMPTY_SET_LIST}" "$TEST8"
    fi
  i=$((i+1))

done < $LOG

echo "################################"
echo "### Exec it tests Grupo List ###"
echo "################################"

TEST0="lpush e 0 1 2 3"
TEST1="lrange e 0 -1"
TEST2="lindex e 2"
TEST3="llen e"
TEST4="llen x"
TEST5="lpop e"
TEST6="lpop e 2"
TEST7="lpop e 1"
TEST8="lpop e"
TEST9="lpop e"
TEST10="lpush x 1 2 3 3 4 3 5"
TEST11="lrem x 3 -1"
TEST12="lrem x 3 0"

#TEST11="lrem e"

rm $LOG
{ 
  echo $TEST0;
  echo $TEST1;
  echo $TEST2;
  echo $TEST3;
  echo $TEST4;
  echo $TEST5;
  echo $TEST6;
  echo $TEST7;
  echo $TEST8;
  echo $TEST9;
  echo $TEST10;
  echo $TEST11;
  echo $TEST12;
  
  echo "exit"
  echo "exit"
  sleep 1;
} | telnet $HOST_REDIS 1>$LOG 2>&1

sleep 1;

i=1
while IFS= read -r line
do
    line=$(echo "$line")
    if [[ $i == "4" ]]; then
    test $i "$line" "${RES_HOST_REDIS}4" "$TEST0"
    elif [[ $i == "5" ]]; then
    test $i "$line" "${RES_HOST_REDIS}0) 3" "$TEST1"
    elif [[ $i == "6" ]]; then
    test $i "$line" "1) 2" "$TEST1"
    elif [[ $i == "7" ]]; then
    test $i "$line" "2) 1" "$TEST1"
    elif [[ $i == "8" ]]; then
    test $i "$line" "3) 0" "$TEST1"
    elif [[ $i == "9" ]]; then
    test $i "$line" "${RES_HOST_REDIS}1" "$TEST2"
    elif [[ $i == "10" ]]; then
    test $i "$line" "${RES_HOST_REDIS}4" "$TEST3"
    elif [[ $i == "11" ]]; then
    test $i "$line" "${RES_HOST_REDIS}0" "$TEST4"
    elif [[ $i == "12" ]]; then
    test $i "$line" "${RES_HOST_REDIS}0) 3" "$TEST5"
    elif [[ $i == "13" ]]; then
    test $i "$line" "${RES_HOST_REDIS}0) 2" "$TEST6"
    elif [[ $i == "14" ]]; then
    test $i "$line" "1) 1" "$TEST6"
    elif [[ $i == "14" ]]; then
    test $i "$line" "${RES_HOST_REDIS}0) 0" "$TEST7"
    elif [[ $i == "15" ]]; then
    test $i "$line" "${RES_HOST_REDIS}0) 0" "$TEST8"
    elif [[ $i == "16" ]]; then
    test $i "$line" "${RES_NIL}" "$TEST9"
    elif [[ $i == "18" ]]; then
    test $i "$line" "${RES_HOST_REDIS}7" "$TEST10"
    elif [[ $i == "19" ]]; then
    test $i "$line" "${RES_HOST_REDIS}1" "$TEST11"
    elif [[ $i == "20" ]]; then
    test $i "$line" "${RES_HOST_REDIS}2" "$TEST12"
    fi
  i=$((i+1))

done < $LOG

kill ${pid_redis}

echo "Exit it test"

TEST11="lrem x 3 -1"
TEST12="lrem x 3 0"