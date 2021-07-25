#!/bin/bash

LOG=/tmp/it_log
#Comienzo a contar el tiempo
START_TIME=$SECONDS

RES_OK="127.0.0.1:8081> OK"
RES_NIL="127.0.0.1:8081> (nil)"
RES_0="127.0.0.1:8081> 0"
RES_1="127.0.0.1:8081> 1"
RES_2="127.0.0.1:8081> 2"
RES_3="127.0.0.1:8081> 3"
RES_12="127.0.0.1:8081> 12"
RES_4="127.0.0.1:8081> 4"

TEST4="set a 1"
TEST5="get a"
TEST6="get b"
TEST7="append a 2"
TEST8='get a'
TEST9="dbsize"
TEST10="flushdb"

### FUNCTIONS ###

function test(){
	i=$1
	line=$2
	to_test=$3
	to_test_in=$4

	PURPLE='\033[0;33m'
	RED='\033[0;31m'
	GREEN='\033[0;32m'
	NC='\033[0m' # No Color
	
	if [[ $line == $to_test ]]; then
		printf "$to_test_in > $line >>>>>> TEST $i ... ${GREEN}ok${NC}\n"
  	else
  		printf "$to_test_in > $line >>>>>> TEST $i ... ${RED}FAILED${NC}. ${PURPLE}Expected $to_test, found $line ${NC}\n"
  	fi
}

### MAIN ###
echo "Exec it tests.."

{ 
	echo $TEST4;
	echo $TEST5;
	echo $TEST6;
	echo $TEST7;
	echo $TEST8;
	echo $TEST9;
	echo $TEST10;
	echo $TEST9;
	sleep 1;
} | telnet 127.0.0.1 8081 1>$LOG 2>&1

sleep 1;
i=1
while IFS= read -r line
do

  line=$(echo "$line")
  #echo "in while"
  #echo "Reading: "$line #" "$i
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
  	test $i "$line" "$RES_2" "$TEST9"
  elif [[ $i == "10" ]]; then
  	test $i "$line" "$RES_OK" "$TEST10"
  elif [[ $i == "11" ]]; then
  	test $i "$line" "$RES_0" "$TEST11"
  fi

  i=$((i+1))

done < $LOG