

PROJECT_NAME=Macrobot
JAR_NAME=$(PROJECT_NAME).jar

MAIN_CLASS=sgf.App

SRC_DIR=src
LIB_DIR=lib
OUT_DIR=bin

SOURCE_FILES = $(shell find src/ -type f -name '*.java')
CLASS_FILES  = $(patsubst $(SRC_DIR)/%.java, $(OUT_DIR)/%.class, $(SOURCE_FILES))

default: build

$(OUT_DIR)/%.class: $(SRC_DIR)/%.java
	javac -d $(OUT_DIR) -classpath $(OUT_DIR):$(SRC_DIR) $<
build-class: $(SOURCE_FILES)
	javac -d $(OUT_DIR) $(SOURCE_FILES)

build: build-class
	jar cfe $(JAR_NAME) $(MAIN_CLASS) -C $(OUT_DIR) .

run: build
	java -jar $(JAR_NAME)

clean:
	rm $(JAR_NAME)
	rm -rf $(OUT_DIR)/*
