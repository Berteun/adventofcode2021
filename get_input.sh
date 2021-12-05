#!/bin/sh
if [ -z "$1" ]; then
  echo "Specify day number." >&2
  exit 1;
fi


DAY=$1
LDAY=$(echo "0${DAY}" | sed 's#^.*\(..\)$#\1#')
DIR="day${LDAY}"
if [ ! -d "${DIR}" ]; then
  cargo new "${DIR}"
  echo '/target' > ${DIR}/.gitignore
fi;

COOKIE=$(cat cookie)
curl -sb "session=${COOKIE}" "https://adventofcode.com/2021/day/${DAY}" | w3m -T text/html -dump > "${DIR}/text"
curl -sb "session=${COOKIE}" "https://adventofcode.com/2021/day/${DAY}/input" -o "${DIR}/input"
