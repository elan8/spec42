#!/usr/bin/env sh

set -eu

if [ "$#" -lt 1 ]; then
  echo "Usage: $0 <input-json>" >&2
  exit 2
fi

INPUT_JSON=$1
SCRIPT_ROOT=$(CDPATH= cd -- "$(dirname "$0")" && pwd)
ROOT=$(CDPATH= cd -- "$SCRIPT_ROOT/.." && pwd)
MODULE_DIR="$ROOT/java/elk-json-runner"
POM="$MODULE_DIR/pom.xml"
TARGET_DIR="$ROOT/target"
MAVEN_REPO_DIR="$TARGET_DIR/m2"
RUN_TOKEN=${RUN_TOKEN:-$$}
MAVEN_SETTINGS="$TARGET_DIR/maven-settings-$RUN_TOKEN.xml"

if [ ! -f "$INPUT_JSON" ]; then
  echo "InputJson not found: $INPUT_JSON" >&2
  exit 1
fi

if [ -n "${JAVA_HOME:-}" ]; then
  export PATH="$JAVA_HOME/bin:$PATH"
fi

if ! command -v java >/dev/null 2>&1; then
  echo "Java not found" >&2
  exit 1
fi

mkdir -p "$TARGET_DIR" "$MAVEN_REPO_DIR"
cat >"$MAVEN_SETTINGS" <<EOF
<settings>
  <localRepository>$MAVEN_REPO_DIR</localRepository>
</settings>
EOF

if command -v mvn >/dev/null 2>&1; then
  MVN_CMD=mvn
elif [ -x "$ROOT/vendor/apache-maven/apache-maven-3.9.9/bin/mvn" ]; then
  MVN_CMD="$ROOT/vendor/apache-maven/apache-maven-3.9.9/bin/mvn"
else
  echo "Maven not found" >&2
  exit 1
fi

echo "Running Java runner via Maven exec:java..." >&2
"$MVN_CMD" -q --batch-mode -f "$POM" \
  -s "$MAVEN_SETTINGS" \
  compile \
  exec:java \
  "-Dexec.args=$INPUT_JSON" \
  "-DskipTests=true" \
  -e
