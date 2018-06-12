#!/bin/sh

SCRIPT="$0"
echo "# START SCRIPT: $SCRIPT"

while [ -h "$SCRIPT" ] ; do
  ls=`ls -ld "$SCRIPT"`
  link=`expr "$ls" : '.*-> \(.*\)$'`
  if expr "$link" : '/.*' > /dev/null; then
    SCRIPT="$link"
  else
    SCRIPT=`dirname "$SCRIPT"`/"$link"
  fi
done

if [ ! -d "${APP_DIR}" ]; then
  APP_DIR=`dirname "$SCRIPT"`/..
  APP_DIR=`cd "${APP_DIR}"; pwd`
fi

executable="./modules/openapi-generator-cli/target/openapi-generator-cli.jar"

if [ ! -f "$executable" ]
then
  mvn -B clean package
fi

export JAVA_OPTS="${JAVA_OPTS} -XX:MaxPermSize=256M -Xmx1024M -DloggerPath=conf/log4j.properties"
test_files=$(ls modules/openapi-generator/src/test/resources/2_0/rust-server/)

for test_file in $test_files; do
  test=$(echo $test_file | sed 's/.yaml//')

  ags="generate -t modules/openapi-generator/src/main/resources/rust-server \
    -i modules/openapi-generator/src/test/resources/2_0/rust-server/$test_file \
    -g rust-server \
    -o samples/server/petstore/rust-server/sample-$test \
    -DpackageName=$test $@"

  java $JAVA_OPTS -jar $executable $ags
done
